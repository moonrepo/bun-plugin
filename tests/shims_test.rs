use proto_pdk_test_utils::*;

#[cfg(not(windows))]
generate_global_shims_test!("bun-test", ["bunx"]);
