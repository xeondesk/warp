use std::fmt;

/// Namespaced secure-storage keys used by local AI providers.
pub mod names {
    pub const OPENROUTER_API_KEY: &str = "ai/openrouter/api_key";
    pub const KILO_API_KEY: &str = "ai/kilo/api_key";
    pub const OPENAI_API_KEY: &str = "ai/openai/api_key";
    pub const ANTHROPIC_API_KEY: &str = "ai/anthropic/api_key";

    pub fn custom_api_key(provider: &str) -> String {
        format!("ai/custom/{provider}/api_key")
    }
}

/// A secret value that never exposes its contents through `Debug` or `Display`.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

/// Storage boundary for provider credentials. Implementations should use the platform
/// secure-storage service and must not serialize credentials with ordinary settings.
pub trait CredentialStore: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<SecretString>, CredentialError>;
    fn set(&self, key: &str, value: SecretString) -> Result<(), CredentialError>;
    fn delete(&self, key: &str) -> Result<(), CredentialError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredentialError {
    Storage(String),
}

impl fmt::Display for CredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(message) => write!(formatter, "credential storage error: {message}"),
        }
    }
}

impl std::error::Error for CredentialError {}

#[cfg(test)]
mod tests {
    use super::{names, SecretString};

    #[test]
    fn secret_debug_and_display_are_redacted() {
        let secret = SecretString::new("sensitive");
        assert_eq!(format!("{secret:?}"), "[REDACTED]");
        assert_eq!(secret.to_string(), "[REDACTED]");
        assert_eq!(secret.expose_secret(), "sensitive");
    }

    #[test]
    fn custom_credentials_are_namespaced() {
        assert_eq!(names::custom_api_key("enterprise"), "ai/custom/enterprise/api_key");
    }
}
