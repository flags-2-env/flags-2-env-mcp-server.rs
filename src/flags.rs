//! Canonical fail-closed argv admission through flags-2-env.

use std::io::Write;

use flags2env::BundledFlags2Env;
use tempfile::NamedTempFile;

const CONTRACT: &str = include_str!("../.cli-flags.toml");

/// Audit the embedded repository-owned flag contract and reject every
/// undeclared option or positional before any MCP transport is initialized.
pub fn preflight() -> Result<(), String> {
    preflight_from(&std::env::args().collect::<Vec<_>>())
}

fn preflight_from(argv: &[String]) -> Result<(), String> {
    let mut contract = NamedTempFile::new()
        .map_err(|_| "cannot materialize embedded flags-2-env contract".to_owned())?;
    contract
        .write_all(CONTRACT.as_bytes())
        .map_err(|_| "cannot materialize embedded flags-2-env contract".to_owned())?;
    let path = contract
        .path()
        .to_str()
        .ok_or_else(|| "flags-2-env contract path is not valid UTF-8".to_owned())?;

    let parser = BundledFlags2Env::new();
    parser
        .audit_config(Some(path))
        .map_err(|_| "flags-2-env contract audit failed".to_owned())?;
    let parsed = parser
        .parse_structured(argv, Some(path))
        .map_err(|_| "flags-2-env parser unavailable".to_owned())?;

    if !parsed.errors.is_empty()
        || !parsed.unknown_options.is_empty()
        || !parsed.extras.is_empty()
        || !parsed.command.is_empty()
        || !parsed.subcommands.is_empty()
    {
        return Err("MCP server command-line arguments are not permitted".to_owned());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| (*item).to_owned()).collect()
    }

    #[test]
    fn no_arguments_pass_the_embedded_contract() {
        preflight_from(&argv(&["flags-2-env-mcp-server"])).expect("no-argument preflight");
    }

    #[test]
    fn unknown_option_fails_without_reflecting_its_value() {
        let marker = "synthetic-secret-never-reflect";
        let error = preflight_from(&argv(&[
            "flags-2-env-mcp-server",
            &format!("--definitely-not-declared={marker}"),
        ]))
        .expect_err("unknown option must fail");
        assert!(!error.contains(marker));
        assert_eq!(error, "MCP server command-line arguments are not permitted");
    }

    #[test]
    fn positional_operand_fails_closed() {
        let error = preflight_from(&argv(&["flags-2-env-mcp-server", "unexpected"]))
            .expect_err("unexpected operand must fail");
        assert_eq!(error, "MCP server command-line arguments are not permitted");
    }
}
