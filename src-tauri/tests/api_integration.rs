#[test]
fn deepseek_endpoint() {
    assert_eq!(app_lib::api::provider_endpoint("deepseek"), "https://api.deepseek.com/v1/chat/completions");
}

#[test]
fn opencode_go_endpoint() {
    assert_eq!(app_lib::api::provider_endpoint("opencode_go"), "https://opencode.ai/zen/go/v1/chat/completions");
}

#[test]
fn opencode_zen_endpoint() {
    assert_eq!(app_lib::api::provider_endpoint("opencode_zen"), "https://opencode.ai/zen/v1/chat/completions");
}

#[test]
fn unknown_provider_falls_back_to_deepseek() {
    assert_eq!(app_lib::api::provider_endpoint("unknown"), "https://api.deepseek.com/v1/chat/completions");
}

#[test]
fn empty_string_falls_back_to_deepseek() {
    assert_eq!(app_lib::api::provider_endpoint(""), "https://api.deepseek.com/v1/chat/completions");
}
