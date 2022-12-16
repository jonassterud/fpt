use crate::structures;
use anyhow::{anyhow, Result};
use rusqlite::{Connection, ToSql};

pub struct Database {
    connection: Option<Connection>,
}

impl Database {
    pub fn open() -> Result<Database> {
        let connection = Connection::open("data.db3")?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS assets (
                id    INTEGER PRIMARY KEY,
                data  BLOB
            )",
            (),
        )?;

        Ok(Database {
            connection: Some(connection),
        })
    }

    pub fn get_connection(&self) -> Result<&Connection> {
        self.connection
            .as_ref()
            .ok_or_else(|| anyhow!("missing connection"))
    }

    pub fn add_asset(&self, asset: &structures::Asset) -> Result<()> {
        self.get_connection()?.execute(
            "INSERT INTO assets (data) VALUES (?1)",
            [&serde_json::to_value(asset)? as &dyn ToSql],
        )?;

        Ok(())
    }

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
}
