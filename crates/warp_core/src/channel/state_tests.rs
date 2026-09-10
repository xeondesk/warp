use super::{derive_http_origin_from_ws_url, ChannelState};

#[test]
fn wss_becomes_https_and_strips_path() {
    let got = derive_http_origin_from_ws_url("wss://rtc.app.warp.dev/graphql/v2");
    assert_eq!(got.as_deref(), Some("https://rtc.app.warp.dev"));
}

#[test]
fn ws_becomes_http_and_preserves_port() {
    let got = derive_http_origin_from_ws_url("ws://localhost:8080/graphql/v2");
    assert_eq!(got.as_deref(), Some("http://localhost:8080"));
}

#[test]
fn unparseable_input_returns_none() {
    assert!(derive_http_origin_from_ws_url("not a url").is_none());
    assert!(derive_http_origin_from_ws_url("https://app.warp.dev").is_none());
}

#[test]
fn is_warp_host_matches_root_and_subdomains() {
    assert!(ChannelState::is_warp_host("warp.dev"));
    assert!(ChannelState::is_warp_host("app.warp.dev"));
    assert!(ChannelState::is_warp_host("oz.warp.dev"));
    assert!(ChannelState::is_warp_host("staging.warp.dev"));
    assert!(!ChannelState::is_warp_host("notwarp.dev"));
    assert!(!ChannelState::is_warp_host("warp.dev.evil.com"));
    assert!(!ChannelState::is_warp_host("example.com"));
}

#[test]
fn is_staging_host_matches_only_staging() {
    assert!(ChannelState::is_staging_host("staging.warp.dev"));
    assert!(!ChannelState::is_staging_host("app.warp.dev"));
    assert!(!ChannelState::is_staging_host("warp.dev"));
}

#[test]
fn legacy_channel_config_without_new_fields_still_parses() {
    // Payloads from older channel-config generators lack `public_urls` and
    // `is_staging`; serde defaults must fill production values.
    let json = r#"{
        "app_id": "dev.warp.WarpOss",
        "logfile_name": "warp-oss.log",
        "server_config": {
            "server_root_url": "https://app.warp.dev",
            "rtc_server_url": "wss://rtc.app.warp.dev/graphql/v2",
            "session_sharing_server_url": "wss://sessions.app.warp.dev",
            "firebase_auth_api_key": "key"
        },
        "oz_config": {"oz_root_url": "https://oz.warp.dev"}
    }"#;
    let config: crate::channel::ChannelConfig =
        serde_json::from_str(json).expect("legacy config should parse");
    assert_eq!(&*config.public_urls.docs_base_url, "https://docs.warp.dev");
    assert_eq!(
        &*config.public_urls.website_base_url,
        "https://www.warp.dev"
    );
    assert!(!config.is_staging);
}

#[test]
fn public_url_helpers_join_paths_and_honor_overrides() {
    // Single test (not several) because overrides mutate the global
    // `CHANNEL_STATE`; no other test in this binary touches these fields.
    // Production defaults.
    assert_eq!(
        ChannelState::docs_base_url().into_owned(),
        "https://docs.warp.dev"
    );
    assert_eq!(
        ChannelState::website_base_url().into_owned(),
        "https://www.warp.dev"
    );
    assert_eq!(
        ChannelState::oz_root_url().into_owned(),
        "https://oz.warp.dev"
    );

    // Leading slashes on the path are tolerated.
    assert_eq!(
        ChannelState::docs_url("terminal/warpify/ssh"),
        "https://docs.warp.dev/terminal/warpify/ssh"
    );
    assert_eq!(
        ChannelState::docs_url("/terminal/warpify/ssh"),
        "https://docs.warp.dev/terminal/warpify/ssh"
    );
    assert_eq!(
        ChannelState::website_url("pricing"),
        "https://www.warp.dev/pricing"
    );
    assert_eq!(
        ChannelState::website_url("/pricing"),
        "https://www.warp.dev/pricing"
    );
    assert_eq!(ChannelState::oz_url(""), "https://oz.warp.dev");
    assert_eq!(ChannelState::oz_url("/runs"), "https://oz.warp.dev/runs");
    assert_eq!(ChannelState::oz_url("runs"), "https://oz.warp.dev/runs");

    // Overrides take effect for derived URLs.
    ChannelState::override_docs_base_url("https://docs.example.com").unwrap();
    ChannelState::override_website_base_url("https://www.example.com").unwrap();
    ChannelState::override_oz_root_url("https://oz.example.com").unwrap();
    assert_eq!(
        ChannelState::docs_url("a/b"),
        "https://docs.example.com/a/b"
    );
    assert_eq!(
        ChannelState::website_url("pricing"),
        "https://www.example.com/pricing"
    );
    assert_eq!(
        ChannelState::oz_url("/runs"),
        "https://oz.example.com/runs"
    );

    // Invalid URLs are rejected and leave state untouched.
    assert!(ChannelState::override_docs_base_url("not a url").is_err());
    assert!(ChannelState::override_website_base_url("not a url").is_err());
    assert!(ChannelState::override_oz_root_url("not a url").is_err());
    assert_eq!(
        ChannelState::docs_url("a/b"),
        "https://docs.example.com/a/b"
    );

    // Restore production defaults so later tests see a clean state.
    ChannelState::override_docs_base_url("https://docs.warp.dev").unwrap();
    ChannelState::override_website_base_url("https://www.warp.dev").unwrap();
    ChannelState::override_oz_root_url("https://oz.warp.dev").unwrap();
    assert_eq!(
        ChannelState::docs_url("a/b"),
        "https://docs.warp.dev/a/b"
    );
}
