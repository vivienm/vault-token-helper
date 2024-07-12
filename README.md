# Vault Token Helper

A [token helper][token-helper] for HashiCorp Vault.

This program lets you store several Vault tokens simultaneously in an SQLite database.
This allows to switch `VAULT_ADDR` back and forth without having to login each time.

[token-helper]: https://developer.hashicorp.com/vault/docs/commands/token-helper

## Installation

Build and install the `vault-token-helper` binary:

```shell
sudo apt install libsqlite3-dev
cargo install --git https://github.com/vivienm/vault-token-helper.git
```

Check it works properly:

```console
$ echo "test" | vault-token-helper store
$ vault-token-helper get
test
$ vault-token-helper erase
```

Configure Vault CLI to use `vault-token-helper`:

```shell
vault-token-helper install --interactive
```

That's it!
You can now use `vault` as usual, and tokens will be stored in the database.

## Database

Vault tokens are stored in an SQLite database, by default in `${XDG_DATA_HOME}/vault-token-helper.db`.
Database location can be overridden with the `VAULT_TOKEN_HELPER_DB` environment variable.

```console
$ sqlite3 ~/.local/share/vault-token-helper.db "SELECT * FROM vault_tokens"
vault_addr              token  created_at
----------------------  -----  -------------------
https://127.0.0.1:8200  XXXXX  2024-06-27 11:25:29
```

## Logging

To configure log verbosity, set the environment variable `VAULT_TOKEN_HELPER_LOG_LEVEL`.

```shell
export VAULT_TOKEN_HELPER_LOG_LEVEL=debug
```
