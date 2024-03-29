use proto_pdk_test_utils::*;

#[cfg(not(windows))]
generate_download_install_tests!("bun-test", "1.0.0");

#[cfg(not(windows))]
mod canary {
    use super::*;

    generate_download_install_tests!("bun-test", "canary");
}

#[test]
fn supports_linux_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("bun-test", |config| {
        config.host(HostOS::Linux, HostArch::Arm64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("bun-linux-aarch64".into()),
            checksum_url: Some(
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/SHASUMS256.txt".into()
            ),
            download_name: Some("bun-linux-aarch64.zip".into()),
            download_url:
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/bun-linux-aarch64.zip"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("bun-test", |config| {
        config.host(HostOS::Linux, HostArch::X64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("bun-linux-x64".into()),
            checksum_url: Some(
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/SHASUMS256.txt".into()
            ),
            download_name: Some("bun-linux-x64.zip".into()),
            download_url:
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/bun-linux-x64.zip"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("bun-test", |config| {
        config.host(HostOS::MacOS, HostArch::Arm64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("bun-darwin-aarch64".into()),
            checksum_url: Some(
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/SHASUMS256.txt".into()
            ),
            download_name: Some("bun-darwin-aarch64.zip".into()),
            download_url:
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/bun-darwin-aarch64.zip"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("bun-test", |config| {
        config.host(HostOS::MacOS, HostArch::X64);
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("bun-darwin-x64".into()),
            checksum_url: Some(
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/SHASUMS256.txt".into()
            ),
            download_name: Some("bun-darwin-x64.zip".into()),
            download_url:
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/bun-darwin-x64.zip"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
#[should_panic(expected = "Unable to install Bun")]
fn doesnt_support_windows() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("bun-test", |config| {
        config.host(HostOS::Windows, HostArch::X64);
    });

    plugin.download_prebuilt(DownloadPrebuiltInput {
        context: ToolContext {
            version: VersionSpec::parse("1.2.0").unwrap(),
            ..Default::default()
        },
        ..Default::default()
    });
}

#[test]
fn locates_unix_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("bun-test", |config| {
        config.host(HostOS::Linux, HostArch::Arm64);
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                }
            })
            .primary
            .unwrap()
            .exe_path,
        Some("bun".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin_with_config("bun-test", |config| {
        config.host(HostOS::Windows, HostArch::X64);
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                }
            })
            .primary
            .unwrap()
            .exe_path,
        Some("bun.exe".into())
    );
}
