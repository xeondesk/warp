use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::AppId;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelConfig {
    /// The application ID for this channel.
    pub app_id: AppId,

    /// The name of the file to which logs should be written.
    pub logfile_name: Cow<'static, str>,

    /// Configuration for talking to Warp's servers.
    pub server_config: WarpServerConfig,
    /// Configuration for Oz/ambient agents.
    pub oz_config: OzConfig,
    /// Public-facing website/docs URLs. Defaults to production when absent
    /// so older channel-config payloads keep parsing.
    #[serde(default)]
    pub public_urls: PublicUrlsConfig,
    /// Whether this channel points at a staging backend. Used instead of
    /// hostname sniffing (`staging.warp.dev`) so custom/staging URLs stay
    /// detectable when `server_root_url` is overridden.
    #[serde(default)]
    pub is_staging: bool,
    /// Configuration for telemetry sending, or [`None`] if telemetry should be
    /// disabled for this build.
    pub telemetry_config: Option<TelemetryConfig>,
    /// Configuration for autoupdate functionality.
    pub autoupdate_config: Option<AutoupdateConfig>,
    /// Configuration for crash reporting.
    pub crash_reporting_config: Option<CrashReportingConfig>,
    /// Configuration for statically-bundled MCP OAuth credentials.
    pub mcp_static_config: Option<McpStaticConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WarpServerConfig {
    /// The root URL for the standard server pool.
    pub server_root_url: Cow<'static, str>,
    /// The URL for the RTC server, which serves real-time updates for Warp Drive objects.
    pub rtc_server_url: Cow<'static, str>,
    /// The URL for the session sharing server, or [`None`] if session sharing is not
    /// supported.
    pub session_sharing_server_url: Option<Cow<'static, str>>,
    /// The API key to use when making requests to Firebase Authentication endpoints.
    pub firebase_auth_api_key: Cow<'static, str>,
}

impl WarpServerConfig {
    pub fn production() -> Self {
        Self {
            server_root_url: "https://app.warp.dev".into(),
            rtc_server_url: "wss://rtc.app.warp.dev/graphql/v2".into(),
            session_sharing_server_url: Some("wss://sessions.app.warp.dev".into()),
            firebase_auth_api_key: "AIzaSyBdy3O3S9hrdayLJxJ7mriBR4qgUaUygAs".into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OzConfig {
    /// Root URL for the Oz (ambient agent management) dashboard.
    pub oz_root_url: Cow<'static, str>,

    /// URL to use as the audience when issuing workload identity tokens. If [`None`], falls back
    /// to [`WarpServerConfig::server_root_url`]. This exists so the audience is not overridden
    /// when a custom server root URL is provided (e.g. an ngrok URL for local development).
    pub workload_audience_url: Option<Cow<'static, str>>,
}

impl OzConfig {
    pub fn production() -> Self {
        Self {
            oz_root_url: "https://oz.warp.dev".into(),
            workload_audience_url: None,
        }
    }
}

/// Public-facing website/docs URLs.
///
/// These are *not* backend URLs: docs and marketing pages stay on production
/// hosts for all channels unless explicitly overridden for development. They
/// live in channel config (instead of scattered string literals) so UI code
/// can build links dynamically via [`crate::channel::ChannelState`].
#[derive(Debug, Deserialize, Serialize)]
pub struct PublicUrlsConfig {
    /// Base URL for user documentation (e.g. `https://docs.warp.dev`).
    #[serde(default = "PublicUrlsConfig::default_docs_base_url")]
    pub docs_base_url: Cow<'static, str>,
    /// Base URL for the marketing website (e.g. `https://www.warp.dev`).
    #[serde(default = "PublicUrlsConfig::default_website_base_url")]
    pub website_base_url: Cow<'static, str>,
}

impl PublicUrlsConfig {
    pub fn production() -> Self {
        Self {
            docs_base_url: "https://docs.warp.dev".into(),
            website_base_url: "https://www.warp.dev".into(),
        }
    }

    fn default_docs_base_url() -> Cow<'static, str> {
        "https://docs.warp.dev".into()
    }

    fn default_website_base_url() -> Cow<'static, str> {
        "https://www.warp.dev".into()
    }
}

impl Default for PublicUrlsConfig {
    fn default() -> Self {
        Self::production()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TelemetryConfig {
    /// The name of the file in which not-yet-sent telemetry events will be stored.
    pub telemetry_file_name: Cow<'static, str>,
    /// Configuration for Rudderstack, for reporting telemetry events.
    pub rudderstack_config: Option<RudderStackConfig>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RudderStackConfig {
    pub write_key: Cow<'static, str>,
    pub root_url: Cow<'static, str>,
    pub ugc_write_key: Cow<'static, str>,
}

impl RudderStackConfig {
    pub fn non_ugc_destination(&self) -> RudderStackDestination {
        RudderStackDestination {
            root_url: self.root_url.clone(),
            write_key: self.write_key.clone(),
        }
    }

    pub fn ugc_destination(&self) -> RudderStackDestination {
        RudderStackDestination {
            root_url: self.root_url.clone(),
            write_key: self.ugc_write_key.clone(),
        }
    }
}

#[derive(Default)]
pub struct RudderStackDestination {
    pub root_url: Cow<'static, str>,
    pub write_key: Cow<'static, str>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AutoupdateConfig {
    /// The base URL for fetching autoupdate versions and updated release bundles.
    pub releases_base_url: Cow<'static, str>,
    /// Whether or not to display menu items relating to autoupdate.
    pub show_autoupdate_menu_items: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrashReportingConfig {
    /// The URL/DSN for sending error logs and crash reports to Sentry.
    pub sentry_url: Cow<'static, str>,
}

/// Configuration for statically-bundled MCP OAuth credentials.
///
/// These are credentials for OAuth providers where dynamic client registration
/// is not supported and we instead ship pre-registered client IDs and secrets.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct McpStaticConfig {
    /// Per-provider OAuth credentials.
    pub providers: Vec<McpOAuthProviderConfig>,
}

/// A single OAuth provider's credentials for MCP authentication.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct McpOAuthProviderConfig {
    /// The issuer URL of the OAuth provider (e.g. `https://github.com/login/oauth`).
    pub issuer: Cow<'static, str>,
    /// The OAuth client ID registered for this channel.
    pub client_id: Cow<'static, str>,
    /// The OAuth client secret registered for this channel.
    pub client_secret: Cow<'static, str>,
}
