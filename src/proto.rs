use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

static NAME: &str = "Bun";
static BIN: &str = "bun";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        self_upgrade_commands: vec!["upgrade".into()],
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/oven-sh/bun")?;

    let tags = tags
        .iter()
        .filter_map(|tag| tag.strip_prefix("bun-v").map(|tag| tag.to_owned()))
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
        ],
    )?;

    let version = &input.context.version;

    let arch = match env.arch {
        HostArch::Arm64 => "aarch64",
        HostArch::X64 => "x64",
        _ => unreachable!(),
    };

    let mut avx2_suffix = "";

    if env.arch == HostArch::X64 && env.os == HostOS::Linux && command_exists(&env, "grep") {
        let output = exec_command!("grep", ["avx2", "/proc/cpuinfo"]);
        if output.exit_code != 0 {
            avx2_suffix = "-baseline";
        }
    };

    let prefix = match env.os {
        HostOS::Linux => format!("bun-linux-{arch}{avx2_suffix}"),
        HostOS::MacOS => format!("bun-darwin-{arch}{avx2_suffix}"),
        _ => unreachable!(),
    };

    let filename = format!("{prefix}.zip");

    let tag = if version.is_canary() {
        "canary".to_owned()
    } else {
        format!("bun-v{version}")
    };

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(prefix),
        download_url: format!("https://github.com/oven-sh/bun/releases/download/{tag}/{filename}"),
        download_name: Some(filename),
        // Checksums are not consistently updated
        checksum_url: if version.is_canary() {
            None
        } else {
            Some(format!(
                "https://github.com/oven-sh/bun/releases/download/{tag}/SHASUMS256.txt"
            ))
        },
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    let bunx = ExecutableConfig {
        // `bunx` isn't a real binary provided by Bun so we can't symlink it.
        // Instead, it's simply the `bun` binary named `bunx` and Bun toggles
        // functionality based on `args[0]`.
        exe_link_path: Some(env.os.get_exe_name(BIN).into()),

        // The approach doesn't work for shims since we use child processes,
        // so execute `bun x` instead (notice the space).
        shim_before_args: Some(StringOrVec::String("x".into())),

        ..ExecutableConfig::default()
    };

    Ok(Json(LocateExecutablesOutput {
        globals_lookup_dirs: vec!["$HOME/.bun/bin".into()],
        primary: Some(ExecutableConfig::new(env.os.get_exe_name(BIN))),
        secondary: HashMap::from_iter([
            // bunx
            ("bunx".into(), bunx),
        ]),
        ..LocateExecutablesOutput::default()
    }))
}
