use zed_extension_api::{self as zed, LanguageServerId};

struct BeancountExtension {}

impl BeancountExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        if let Some(path) = worktree.which("beancount-language-server") {
            return Ok(path);
        }

        return Err("Beancount language server not installed".to_string());
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
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(BeancountExtension);
