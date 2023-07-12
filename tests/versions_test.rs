use proto_pdk::*;
use proto_pdk_test_utils::{create_plugin, generate_resolve_versions_tests};
use starbase_sandbox::create_empty_sandbox;

generate_resolve_versions_tests!("bun-test", {
    "0.4" => "0.4.0",
    "0.5.1" => "0.5.1",
});

#[test]
fn loads_versions_from_git() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(!output.versions.is_empty());
}

#[test]
fn sets_latest_alias() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}
