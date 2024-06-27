mod hcl;
mod install;
mod store;

use std::{
    borrow::Cow,
    io::{self, Read},
    path::PathBuf,
};

use install::install;

use self::store::Store;

#[derive(Debug, clap::Parser)]
#[clap(about)]
struct Args {
    /// The address of the Vault server.
    #[clap(long, env = "VAULT_ADDR", default_value = "https://127.0.0.1:8200")]
    vault_addr: String,
    /// The path to the SQLite database.
    #[clap(long = "db", value_name = "DB", env = "VAULT_TOKEN_HELPER_DB")]
    store_path: Option<PathBuf>,
    /// Set the verbosity level for log messages.
    #[clap(
        short,
        long,
        default_value = "info",
        env = "VAULT_TOKEN_HELPER_LOG_LEVEL"
    )]
    log_level: tracing::level_filters::LevelFilter,
    /// The command to run.
    #[clap(subcommand)]
    command: Command,
}

fn default_store_path() -> anyhow::Result<PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::new()?;
    let store_path = xdg_dirs.place_data_file("vault-token-helper.db")?;
    tracing::debug!(db = %store_path.display(), "Using default database");
    Ok(store_path)
}

#[derive(Debug, clap::Parser)]
enum Command {
    /// Configure Vault to use the token helper.
    Install {
        /// Overwrite the existing configuration.
        #[clap(short, long)]
        force: bool,
        /// Prompt before overwriting the existing configuration.
        #[clap(short, long)]
        interactive: bool,
    },
    /// Show a stored token.
    Get,
    /// Store a token.
    ///
    /// If no token is provided, the token will be read from standard input.
    Store {
        /// The token to store.
        token: Option<String>,
    },
    /// Erase a stored token.
    Erase,
}

fn setup_logging(log_level: tracing::level_filters::LevelFilter) -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

fn command_install(force: bool, interactive: bool) -> anyhow::Result<()> {
    install(force, interactive)
}

fn command_get(store: &Store, vault_addr: &str) -> anyhow::Result<()> {
    match store.get(vault_addr)? {
        Some(token) => {
            tracing::debug!(vault_addr, "Token found");
            print!("{}", token); // Vault won't accept a trailing newline.
        }
        None => {
            tracing::debug!(vault_addr, "Token not found");
        }
    }
    Ok(())
}

fn command_store(store: &Store, vault_addr: &str, token: Option<&str>) -> anyhow::Result<()> {
    let token: Cow<str> = match token {
        Some(token) => token.into(),
        None => {
            let mut token = String::new();
            io::stdin().read_to_string(&mut token)?;
            token.into()
        }
    };
    store.store(vault_addr, &token)?;
    tracing::debug!(vault_addr, "Token stored");
    Ok(())
}

fn command_erase(store: &Store, vault_addr: &str) -> anyhow::Result<()> {
    store.erase(vault_addr)?;
    tracing::debug!(vault_addr, "Token erased");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();
    setup_logging(args.log_level)?;

    let store_path = match &args.store_path {
        Some(store_path) => store_path.as_path(),
        None => &default_store_path()?,
    };
    let store = store::Store::open(store_path)?;

    match args.command {
        Command::Install { force, interactive } => {
            command_install(force, interactive)?;
        }
        Command::Get => {
            command_get(&store, &args.vault_addr)?;
        }
        Command::Store { token } => {
            command_store(&store, &args.vault_addr, token.as_deref())?;
        }
        Command::Erase => {
            command_erase(&store, &args.vault_addr)?;
        }
    }

    Ok(())
}
