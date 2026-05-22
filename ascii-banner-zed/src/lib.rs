use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, GithubReleaseOptions,
    LanguageServerId, Os, Result,
};

struct AsciiBannerExtension;

impl zed::Extension for AsciiBannerExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = worktree.which("ascii-banner-lsp") {
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: Default::default(),
            });
        }

        let (os, arch) = zed::current_platform();
        let server_dir = format!("language_servers/{language_server_id}");
        let binary_name = match os {
            Os::Windows => format!("{server_dir}/ascii-banner-lsp.exe"),
            _ => format!("{server_dir}/ascii-banner-lsp"),
        };

        if !std::path::Path::new(&binary_name).exists() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::CheckingForUpdate,
            );

            let release = zed::latest_github_release(
                "rubjo/ascii-banner-zed",
                GithubReleaseOptions {
                    require_assets: true,
                    pre_release: false,
                },
            )?;

            let asset_arch = match arch {
                Architecture::X8664 => "x86_64",
                Architecture::Aarch64 => "aarch64",
                Architecture::X86 => "x86",
            };
            let asset_os = match os {
                Os::Mac => "apple-darwin",
                Os::Linux => "unknown-linux-gnu",
                Os::Windows => "pc-windows-msvc",
            };
            let asset_name = format!("ascii-banner-lsp-{asset_arch}-{asset_os}.tar.gz");

            let asset = release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .ok_or_else(|| {
                    format!(
                        "no asset found for {asset_name} in release {}",
                        release.version
                    )
                })?;

            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &format!("{server_dir}/{asset_name}"),
                DownloadedFileType::GzipTar,
            )?;

            zed::make_file_executable(&binary_name)?;
        }

        Ok(zed::Command {
            command: binary_name,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(AsciiBannerExtension);
