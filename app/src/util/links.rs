use crate::channel::ChannelState;

#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub const GITHUB_ISSUES_URL: &str = "https://github.com/warpdotdev/Warp/issues";
pub const SLACK_URL: &str = "http://go.warp.dev/join-preview";

/// Legacy constant kept for callers that haven't migrated to [`docs_base_url`].
/// Prefer [`docs_base_url`] / [`docs_url`] for new code.
pub const USER_DOCS_URL: &str = "https://docs.warp.dev/";

/// Legacy constant kept for callers that haven't migrated to [`privacy_policy_url`].
pub const PRIVACY_POLICY_URL: &str = "https://www.warp.dev/privacy";

/// Dynamic docs base URL from channel config (defaults to production).
pub fn docs_base_url() -> String {
    ChannelState::docs_base_url().into_owned()
}

/// Builds a docs URL from a path relative to the docs root.
pub fn docs_url(path: &str) -> String {
    ChannelState::docs_url(path)
}

/// Dynamic website base URL from channel config (defaults to production).
pub fn website_base_url() -> String {
    ChannelState::website_base_url().into_owned()
}

/// Builds a website URL from a path relative to the website root.
pub fn website_url(path: &str) -> String {
    ChannelState::website_url(path)
}

pub fn privacy_policy_url() -> String {
    ChannelState::website_url("privacy")
}

pub fn oz_root_url() -> String {
    ChannelState::oz_root_url().into_owned()
}

pub fn feedback_form_url() -> String {
    let mut url = url::Url::parse("https://github.com/warpdotdev/Warp/issues/new/choose")
        .expect("Should not fail to parse");
    if let Some(version) = ChannelState::app_version() {
        url.query_pairs_mut().append_pair("warp-version", version);
    }
    url.query_pairs_mut()
        .append_pair("os-version", &os_info::get().version().to_string());
    url.to_string()
}
