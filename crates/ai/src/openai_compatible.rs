use crate::provider::{inference::{CompletionStream, InferenceProvider}, CompletionRequest, CompletionResponse, Endpoint, InferenceError, ModelInfo};
use async_trait::async_trait;
use futures::{stream, StreamExt};
use reqwest::{Client, StatusCode};
use std::collections::BTreeMap;

pub struct OpenAiCompatibleProvider {
    client: Client,
    endpoint: Endpoint,
    api_key: Option<String>,
    headers: BTreeMap<String, String>,
}

impl OpenAiCompatibleProvider {
    pub fn new(endpoint: Endpoint, api_key: Option<String>, headers: BTreeMap<String, String>) -> Result<Self, InferenceError> {
        let client = Client::builder().build().map_err(|_| InferenceError::Network)?;
        Ok(Self { client, endpoint, api_key, headers })
    }

    fn request(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let request = self.api_key.as_ref().map_or(request, |key| request.bearer_auth(key));
        self.headers.iter().fold(request, |request, (name, value)| request.header(name, value))
    }

    fn status_error(status: StatusCode) -> InferenceError {
        match status {
            StatusCode::UNAUTHORIZED => InferenceError::AuthenticationFailed,
            StatusCode::FORBIDDEN => InferenceError::AuthorizationFailed,
            StatusCode::NOT_FOUND => InferenceError::ModelNotFound,
            StatusCode::TOO_MANY_REQUESTS => InferenceError::RateLimited,
            s if s.is_server_error() => InferenceError::ProviderUnavailable,
            _ => InferenceError::InvalidRequest,
        }
    }
}

#[async_trait]
impl InferenceProvider for OpenAiCompatibleProvider {
    async fn complete(&self, mut request: CompletionRequest) -> Result<CompletionResponse, InferenceError> {
        request.stream = false;
        let response = self.request(self.client.post(self.endpoint.join("chat/completions")).json(&request)).send().await.map_err(|e| if e.is_timeout() { InferenceError::Timeout } else { InferenceError::Network })?;
        if !response.status().is_success() { return Err(Self::status_error(response.status())); }
        let value: serde_json::Value = response.json().await.map_err(|_| InferenceError::InvalidResponse)?;
        let choice = value.get("choices").and_then(|v| v.as_array()).and_then(|v| v.first()).ok_or(InferenceError::InvalidResponse)?;
        let content = choice.get("message").and_then(|v| v.get("content")).and_then(|v| v.as_str()).ok_or(InferenceError::InvalidResponse)?;
        Ok(CompletionResponse { id: value.get("id").and_then(|v| v.as_str()).map(str::to_owned), model: value.get("model").and_then(|v| v.as_str()).map(str::to_owned), content: content.to_owned(), input_tokens: value.pointer("/usage/prompt_tokens").and_then(|v| v.as_u64()), output_tokens: value.pointer("/usage/completion_tokens").and_then(|v| v.as_u64()) })
    }

    async fn stream(&self, mut request: CompletionRequest) -> Result<CompletionStream, InferenceError> {
        request.stream = true;
        let response = self.request(self.client.post(self.endpoint.join("chat/completions")).json(&request)).send().await.map_err(|_| InferenceError::Network)?;
        if !response.status().is_success() { return Err(Self::status_error(response.status())); }
        let chunks = response.bytes_stream().map(|chunk| chunk.map_err(|_| InferenceError::Network).and_then(|chunk| String::from_utf8(chunk.to_vec()).map_err(|_| InferenceError::InvalidResponse)));
        Ok(Box::pin(chunks.flat_map(|chunk| match chunk { Ok(text) => stream::iter(text.lines().filter_map(|line| line.strip_prefix("data: ").filter(|data| *data != "[DONE]").map(str::to_owned)).map(Ok).collect::<Vec<_>>()), Err(error) => stream::iter(vec![Err(error)]), })))
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>, InferenceError> {
        let response = self.request(self.client.get(self.endpoint.join("models"))).send().await.map_err(|_| InferenceError::Network)?;
        if !response.status().is_success() { return Err(Self::status_error(response.status())); }
        let value: serde_json::Value = response.json().await.map_err(|_| InferenceError::InvalidResponse)?;
        Ok(value.get("data").and_then(|v| v.as_array()).ok_or(InferenceError::InvalidResponse)?.iter().filter_map(|model| model.get("id").and_then(|id| id.as_str()).map(|id| ModelInfo { id: id.to_owned(), context_window: None, max_output_tokens: None, supports_tools: true, supports_streaming: true })).collect())
    }
}
