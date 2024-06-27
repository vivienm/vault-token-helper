use std::{env, fs::File, io::Write, path::PathBuf};

use home::home_dir;

use crate::hcl;

pub fn install(force: bool, interactive: bool) -> anyhow::Result<()> {
    let exe_path = env::current_exe()?;
    let exe_path_str = exe_path
        .as_os_str()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid executable path: {}", exe_path.display()))?;

    let vault_path = vault_config_path()?;

    if !force
        && vault_path.exists()
        && (!interactive
            || !dialoguer::Confirm::new()
                .with_prompt("Vault configuration file already exists. Overwrite?")
                .interact()?)
    {
        anyhow::bail!("Vault configuration file already exists");
    }

    let mut vault_file = File::create(vault_path)?;
    writeln!(vault_file, "# This file was created by vault-token-helper.")?;
    writeln!(
        vault_file,
        "token_helper = \"{}\"",
        hcl::escape_quoted_string(exe_path_str)
    )?;
    Ok(())
}

/// Returns the path to the Vault configuration file.
///
/// If the `VAULT_CONFIG_PATH` environment variable is set, its value is
/// returned. Otherwise, the default path is `$HOME/.vault`.
fn vault_config_path() -> anyhow::Result<PathBuf> {
    // Reference implementation:
    // https://github.com/hashicorp/vault/blob/5f4a0deb77990ad843369d843beacda17a8276fa/api/cliconfig/config.go#L16-L23
    if let Some(config_path) = env::var_os("VAULT_CONFIG_PATH") {
        return Ok(config_path.into());
    }
    let mut config_path =
        home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    config_path.push(".vault");
    Ok(config_path)
}
