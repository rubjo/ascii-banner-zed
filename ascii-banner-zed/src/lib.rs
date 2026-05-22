use zed_extension_api::{self as zed, LanguageServerId, Result};

struct AsciiBannerExtension;

impl zed::Extension for AsciiBannerExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = worktree.which("ascii-banner-lsp") {
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: Default::default(),
            });
        }

        Err("ascii-banner-lsp not found in PATH. Build it first:\n    cd ascii-banner-lsp && cargo build --release\nThen add to PATH:\n    export PATH=\"$PWD/ascii-banner-lsp/target/release:$PATH\"".to_string())
    }
}

zed::register_extension!(AsciiBannerExtension);
