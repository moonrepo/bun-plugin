use extism_pdk::*;
use proto_pdk::*;
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
                arch: other.to_string(),
            })?;
        }
    };

    let prefix = match input.env.os {
        HostOS::Linux => format!("bun-linux-{arch}"),
        HostOS::MacOS => format!("bun-darwin-{arch}"),
        other => {
            return Err(PluginError::UnsupportedPlatform {
                tool: NAME.into(),
                platform: other.to_string(),
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
            BIN.to_owned()
        }),
        fallback_last_globals_dir: true,
        globals_lookup_dirs: vec!["$HOME/.bun/bin".into()],
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/oven-sh/bun")?;

    let tags = tags
        .iter()
        .filter_map(|t| t.strip_prefix("bun-v").map(|t| t.to_owned()))
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from_tags(&tags)?))
}

#[plugin_fn]
pub fn create_shims(Json(_): Json<CreateShimsInput>) -> FnResult<Json<CreateShimsOutput>> {
    let mut global_shims = HashMap::new();
    global_shims.insert("bunx".into(), ShimConfig::global_with_sub_command("x"));

    Ok(Json(CreateShimsOutput {
        global_shims,
        ..CreateShimsOutput::default()
    }))
}
