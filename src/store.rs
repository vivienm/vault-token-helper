use rusqlite::OptionalExtension;
use thiserror::Error;

refinery::embed_migrations!("src/migrations");

#[derive(Debug)]
pub struct Store {
    conn: rusqlite::Connection,
}

impl Store {
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let conn = rusqlite::Connection::open(path)?;
        Self::with_conn(conn)
    }

    fn with_conn(mut conn: rusqlite::Connection) -> Result<Self> {
        migrations::runner().run(&mut conn)?;
        Ok(Self { conn })
    }

    pub fn get(&self, vault_addr: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT token FROM vault_tokens WHERE vault_addr = ?")?;
        let token = stmt
            .query_row(rusqlite::params![vault_addr], |row| row.get("token"))
            .optional()?;
        Ok(token)
    }

    pub fn store(&self, vault_addr: &str, token: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO vault_tokens (vault_addr, token) VALUES (?, ?)
            ON CONFLICT(vault_addr) DO UPDATE SET token = ?, created_at = CURRENT_TIMESTAMP",
            rusqlite::params![vault_addr, token, token],
        )?;
        Ok(())
    }

    pub fn erase(&self, vault_addr: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM vault_tokens WHERE vault_addr = ?",
            rusqlite::params![vault_addr],
        )?;
        Ok(())
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Rusqlite(#[from] rusqlite::Error),
    Refinery(#[from] refinery::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
