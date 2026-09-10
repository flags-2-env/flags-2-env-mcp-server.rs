//! Fleet-generated organization MCP server entry point.

mod flags;
mod spec;

use ore_mcp_org_server::run_stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    flags::preflight().map_err(std::io::Error::other)?;
    run_stdio(spec::org_spec()).await
}
