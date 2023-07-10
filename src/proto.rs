use extism_pdk::*;
use proto_pdk::*;
use serde::Deserialize;
use std::collections::HashMap;

static NAME: &str = "Bun";
static BIN: &str = "bun";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let version = input.env.version;

    let arch = match input.env.arch {
        HostArch::Arm64 => "aarch64",
        HostArch::X64 => "x64",
        other => {
            return Err(PluginError::UnsupportedArchitecture {
                tool: NAME.into(),
                arch: format!("{:?}", other),
            })?;
        }
    };

    let prefix = match input.env.os {
        HostOS::Linux => format!("bun-linux-{arch}"),
        HostOS::MacOS => format!("bun-darwin-{arch}"),
        other => {
            return Err(PluginError::UnsupportedPlatform {
                tool: NAME.into(),
                platform: format!("{:?}", other),
            })?;
        }
    };

    let filename = format!("{prefix}.zip");

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(prefix),
        download_url: format!(
            "https://github.com/oven-sh/bun/releases/download/bun-v{version}/{filename}"
        ),
        download_name: Some(filename),
        checksum_url: Some(format!(
            "https://github.com/oven-sh/bun/releases/download/bun-v{version}/SHASUMS256.txt"
        )),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_bins(Json(input): Json<LocateBinsInput>) -> FnResult<Json<LocateBinsOutput>> {
    Ok(Json(LocateBinsOutput {
        bin_path: Some(if input.env.os == HostOS::Windows {
            format!("{}.exe", BIN) // Not supported yet
        } else {
            format!("{}", BIN)
        }),
        globals_lookup_dirs: vec!["$HOME/.bun/bin".into()],
    }))
}

#[derive(Deserialize)]
pub struct TagEntry {
    name: String,
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let mut output = LoadVersionsOutput::default();
    let mut latest = Version::new(0, 0, 0);

    let response: Vec<TagEntry> = fetch_url("https://api.github.com/repos/oven-sh/bun/tags")?;
    let tags = response
        .iter()
        .filter_map(|entry| entry.name.strip_prefix("bun-v"))
        .collect::<Vec<_>>();

    for tag in tags {
        let version = Version::parse(tag)?;

        if version > latest {
            latest = version.clone();
        }

        output.versions.push(version);
    }

    output.aliases.insert("latest".into(), latest);

    Ok(Json(output))
}

#[plugin_fn]
pub fn create_shims(Json(_): Json<CreateShimsInput>) -> FnResult<Json<CreateShimsOutput>> {
    let mut global_shims = HashMap::new();

    global_shims.insert(
        "bunx".into(),
        ShimConfig {
            before_args: Some("x".into()),
            ..ShimConfig::default()
        },
    );

    Ok(Json(CreateShimsOutput {
        global_shims,
        ..CreateShimsOutput::default()
    }))
}
