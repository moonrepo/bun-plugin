use proto_pdk::*;
use proto_pdk_test_utils::{create_plugin, generate_download_install_tests};
use starbase_sandbox::create_empty_sandbox;
use std::path::PathBuf;

#[cfg(not(windows))]
generate_download_install_tests!("bun-test", "0.6.0");

#[test]
fn supports_linux_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::Arm64,
                os: HostOS::Linux,
                version: "1.2.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("bun-linux-aarch64".into()),
            checksum_name: None,
            checksum_url: Some(
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/SHASUMS256.txt".into()
            ),
            download_name: Some("bun-linux-aarch64.zip".into()),
            download_url:
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/bun-linux-aarch64.zip"
                    .into()
        }
    );
}

#[test]
fn supports_linux_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::X64,
                os: HostOS::Linux,
                version: "1.2.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("bun-linux-x64".into()),
            checksum_name: None,
            checksum_url: Some(
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/SHASUMS256.txt".into()
            ),
            download_name: Some("bun-linux-x64.zip".into()),
            download_url:
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/bun-linux-x64.zip"
                    .into()
        }
    );
}

#[test]
fn supports_macos_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::Arm64,
                os: HostOS::MacOS,
                version: "1.2.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("bun-darwin-aarch64".into()),
            checksum_name: None,
            checksum_url: Some(
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/SHASUMS256.txt".into()
            ),
            download_name: Some("bun-darwin-aarch64.zip".into()),
            download_url:
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/bun-darwin-aarch64.zip"
                    .into()
        }
    );
}

#[test]
fn supports_macos_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::X64,
                os: HostOS::MacOS,
                version: "1.2.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: Some("bun-darwin-x64".into()),
            checksum_name: None,
            checksum_url: Some(
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/SHASUMS256.txt".into()
            ),
            download_name: Some("bun-darwin-x64.zip".into()),
            download_url:
                "https://github.com/oven-sh/bun/releases/download/bun-v1.2.0/bun-darwin-x64.zip"
                    .into()
        }
    );
}

#[test]
#[should_panic(expected = "Unable to install Bun, unsupported platform windows.")]
fn doesnt_support_windows() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    plugin.download_prebuilt(DownloadPrebuiltInput {
        env: Environment {
            arch: HostArch::X64,
            os: HostOS::Windows,
            version: "1.2.0".into(),
            ..Default::default()
        },
    });
}

#[test]
fn locates_unix_bin() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                env: Environment {
                    arch: HostArch::Arm64,
                    os: HostOS::Linux,
                    version: "1.2.0".into(),
                    ..Default::default()
                },
                home_dir: PathBuf::new(),
                tool_dir: PathBuf::new()
            })
            .bin_path,
        Some("bun".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                env: Environment {
                    arch: HostArch::X64,
                    os: HostOS::Windows,
                    version: "1.2.0".into(),
                    ..Default::default()
                },
                home_dir: PathBuf::new(),
                tool_dir: PathBuf::new()
            })
            .bin_path,
        Some("bun.exe".into())
    );
}
