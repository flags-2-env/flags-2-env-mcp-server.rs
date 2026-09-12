//! Shared Auth-protected Streamable HTTP entry point.

mod flags;
mod spec;

use ore_mcp_org_server::run_http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    flags::preflight().map_err(std::io::Error::other)?;
    run_http(spec::org_spec()).await
}
