use zed_extension_api::{self as zed, LanguageServerId, settings::LspSettings};

struct BeancountExtension {}

impl BeancountExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        // First, check if user has configured a custom binary path in settings
        if let Ok(lsp_settings) = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            && let Some(binary_settings) = lsp_settings.binary
            && let Some(path) = binary_settings.path
        {
            return Ok(path);
        }

        // Fall back to searching for binary on PATH
        if let Some(path) = worktree.which("beancount-language-server") {
            return Ok(path);
        }

        Err("Beancount language server not found. Please install it or configure the binary path in settings.".to_string())
    }
}
impl zed::Extension for BeancountExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        BeancountExtension {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let command = self.language_server_binary_path(language_server_id, worktree)?;

        // Check for custom arguments and environment variables in settings
        let (args, env) = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary)
            .map(|binary_settings| {
                let args = binary_settings.arguments.unwrap_or_default();
                let env = binary_settings
                    .env
                    .unwrap_or_default()
                    .into_iter()
                    .collect::<Vec<_>>();
                (args, env)
            })
            .unwrap_or_default();

        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(BeancountExtension);
