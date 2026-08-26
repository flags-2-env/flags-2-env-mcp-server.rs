//! Fleet-generated organization MCP server entry point.

use ore_mcp_org_server::{run_stdio, OrgSpec};

const DEPENDENCIES: &[&str] = &[
    "ORESoftware/mcp-rust-libs",
    "ores-otel/ores-mcp-server-core-libs.rs",
    "shared-auth/shared-auth-clients",
    "shared-auth/shared-auth-interfaces",
    "shared-auth/shared-auth-lib",
    "zed-pkg/zed-cli",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_stdio(OrgSpec {
        organization: "flags-2-env",
        repository: "flags-2-env/flags-2-env-mcp-server.rs",
        service_name: "flags-2-env-mcp-server",
        package_name: "flags-2-env-mcp-server",
        dependencies: DEPENDENCIES,
        version: env!("CARGO_PKG_VERSION"),
    })
    .await
}
