use crate::structures;
use anyhow::{anyhow, Result};
use rusqlite::{Connection, ToSql};

/// SQLite Database.
pub struct Database {
    /// Connection to database.
    connection: Option<Connection>,
}

impl Database {
    /// Open/init database at `data.db3`.
    pub fn open() -> Result<Database> {
        let connection = Connection::open("data.db3")?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS assets (
                id    INTEGER PRIMARY KEY,
                data  BLOB
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS pits (
                id    INTEGER PRIMARY KEY,
                data  BLOB
            )",
            [],
        )?;

        Ok(Database {
            connection: Some(connection),
        })
    }

    /// Get a reference to database connection.
    pub fn get_connection(&self) -> Result<&Connection> {
        self.connection
            .as_ref()
            .ok_or_else(|| anyhow!("missing connection"))
    }

    /// Add [`structures::Asset`] to database.
    pub fn add_asset(&self, asset: &structures::Asset) -> Result<()> {
        self.get_connection()?.execute(
            "INSERT INTO assets (data) VALUES (?1)",
            [&serde_json::to_value(asset)? as &dyn ToSql],
        )?;

        Ok(())
    }

    /// Get [`Vec<structures::Asset>`] from database.
    pub fn get_assets(&self) -> Result<Vec<structures::Asset>> {
        let mut out = vec![];

        let mut stmt = self.get_connection()?.prepare("SELECT data FROM assets")?;
        let assets_iter = stmt.query_map([], |row| {
            let json_value = row.get::<usize, serde_json::Value>(0)?;
            let asset_value = serde_json::from_value::<structures::Asset>(json_value);

            Ok(asset_value)
        })?;

        for asset in assets_iter {
            out.push(asset??);
        }

        Ok(out)
    }

    /// Clear [`structures::Asset`]'s from database.
    pub fn clear_assets(&self) -> Result<()> {
        self.get_connection()?.execute("DELETE FROM assets", [])?;

        Ok(())
    }

    /// Add [`structures::Pit`] to database.
    pub fn add_pit(&self, pit: structures::Pit) -> Result<()> {
        self.get_connection()?.execute(
            "INSERT INTO pits (data) VALUES (?1)",
            [&serde_json::to_value(pit)? as &dyn ToSql],
        )?;

        Ok(())
    }

    /// Get [`Vec<structures::Asset>`] from database.
    pub fn get_pits(&self) -> Result<Vec<structures::Pit>> {
        let mut out = vec![];

        let mut stmt = self.get_connection()?.prepare("SELECT data FROM pits")?;
        let assets_iter = stmt.query_map([], |row| {
            let json_value = row.get::<usize, serde_json::Value>(0)?;
            let asset_value = serde_json::from_value::<structures::Pit>(json_value);

            Ok(asset_value)
        })?;

        for asset in assets_iter {
            out.push(asset??);
        }

        Ok(out)
    }

    /// Remove [`structures::Pit`] from database by id.
    pub fn remove_pit(&self, id: usize) -> Result<()> {
        self.get_connection()?
            .execute("DELETE FROM pits WHERE id = ?1", [id])?;

        Ok(())
    }
}
