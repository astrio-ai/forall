mod server;

use std::path::Path;

pub use server::AuthorMcpServer;
pub use server::run_stdio;

pub fn client_config_json(root: &Path) -> anyhow::Result<String> {
    let root = root.canonicalize()?;
    Ok(serde_json::to_string_pretty(&serde_json::json!({
        "mcpServers": {
            "forall-author": {
                "command": "forall",
                "args": [
                    "mcp-author",
                    "--root",
                    root,
                ],
            },
        },
    }))?)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn client_config_binds_the_canonical_workspace() {
        let root = tempfile::tempdir().expect("tempdir");
        let config = client_config_json(root.path()).expect("config");
        let parsed: serde_json::Value = serde_json::from_str(&config).expect("json");
        assert_eq!(
            parsed["mcpServers"]["forall-author"]["command"],
            serde_json::json!("forall")
        );
        assert_eq!(
            parsed["mcpServers"]["forall-author"]["args"][2],
            serde_json::json!(root.path().canonicalize().expect("canonical"))
        );
    }
}
