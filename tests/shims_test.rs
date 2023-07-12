use proto_pdk_test_utils::{create_plugin, generate_global_shims_test};
use starbase_sandbox::{assert_snapshot, create_empty_sandbox};

#[cfg(not(windows))]
generate_global_shims_test!("bun-test", ["bunx"]);
