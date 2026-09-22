use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProviderId(pub String);

impl fmt::Display for ProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelRef {
    pub provider: ProviderId,
    pub model: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Endpoint {
    pub base_url: String,
}

impl Endpoint {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into().trim_end_matches('/').to_owned() }
    }

    pub fn join(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CredentialRef { pub name: String }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub context_window: Option<u64>,
    pub max_output_tokens: Option<u64>,
    pub supports_tools: bool,
    pub supports_streaming: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderProtocol { OpenAiCompatible, AnthropicMessages, Gemini, Custom }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<CompletionMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u64>,
    pub stream: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionMessage { pub role: String, pub content: String }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: Option<String>,
    pub model: Option<String>,
    pub content: String,
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: ProviderId,
    pub protocol: ProviderProtocol,
    pub endpoint: Endpoint,
    pub credential: Option<CredentialRef>,
    pub headers: BTreeMap<String, String>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InferenceError {
    #[error("AI provider is not configured")]
    NotConfigured,
    #[error("provider authentication failed")]
    AuthenticationFailed,
    #[error("provider authorization failed")]
    AuthorizationFailed,
    #[error("invalid provider request")]
    InvalidRequest,
    #[error("model was not found")]
    ModelNotFound,
    #[error("provider rate limit reached")]
    RateLimited,
    #[error("provider spending limit reached")]
    SpendingLimit,
    #[error("model context window exceeded")]
    ContextWindowExceeded,
    #[error("model output limit exceeded")]
    OutputLimitExceeded,
    #[error("provider unavailable")]
    ProviderUnavailable,
    #[error("provider request timed out")]
    Timeout,
    #[error("provider network request failed")]
    Network,
    #[error("provider returned an invalid response")]
    InvalidResponse,
    #[error("provider capability is unsupported")]
    UnsupportedCapability,
}

pub mod credential;

pub mod inference {
    use super::{CompletionRequest, CompletionResponse, InferenceError, ModelInfo};
    use async_trait::async_trait;
    use futures::stream::BoxStream;

    pub type CompletionStream = BoxStream<'static, Result<String, InferenceError>>;

    #[async_trait]
    pub trait InferenceProvider: Send + Sync {
        async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse, InferenceError>;
        async fn stream(&self, request: CompletionRequest) -> Result<CompletionStream, InferenceError>;
        async fn list_models(&self) -> Result<Vec<ModelInfo>, InferenceError>;
    }
}

pub mod routing {
    use super::{inference::InferenceProvider, InferenceError, ModelRef};
    use std::collections::HashMap;
    use std::sync::Arc;

    #[derive(Default)]
    pub struct InferenceRouter { providers: HashMap<String, Arc<dyn InferenceProvider>> }

    impl InferenceRouter {
        pub fn register(&mut self, provider: impl Into<String>, inference: Arc<dyn InferenceProvider>) {
            self.providers.insert(provider.into(), inference);
        }

        pub fn provider(&self, model: &ModelRef) -> Result<Arc<dyn InferenceProvider>, InferenceError> {
            self.providers.get(&model.provider.0).cloned().ok_or(InferenceError::NotConfigured)
        }
    }
}

pub mod providers {
    use super::{Endpoint, ProviderConfig, ProviderId, ProviderProtocol};

    pub fn openrouter() -> ProviderConfig {
        ProviderConfig { id: ProviderId("openrouter".into()), protocol: ProviderProtocol::OpenAiCompatible, endpoint: Endpoint::new("https://openrouter.ai/api/v1"), credential: Some(super::CredentialRef { name: credential::names::OPENROUTER_API_KEY.into() }), headers: Default::default() }
    }

    pub fn kilo() -> ProviderConfig {
        ProviderConfig { id: ProviderId("kilo".into()), protocol: ProviderProtocol::OpenAiCompatible, endpoint: Endpoint::new("https://api.kilo.ai/api/gateway"), credential: Some(super::CredentialRef { name: credential::names::KILO_API_KEY.into() }), headers: Default::default() }
    }
}
