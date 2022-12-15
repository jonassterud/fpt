use crate::structures;
use anyhow::{anyhow, Result};

pub struct Database {
    connection: Option<sqlite::Connection>,
}

impl Database {
    pub fn open() -> Result<Database> {
        let connection = sqlite::open("database.db")?;

        let query = "CREATE TABLE IF NOT EXISTS assets (data BLOB);";
        connection.execute(query)?;

        Ok(Database {
            connection: Some(connection),
        })
    }

    pub fn close(&mut self) {
        self.connection = None;
    }

    pub fn get_connection(&self) -> Result<&sqlite::Connection> {
        self.connection
            .as_ref()
            .ok_or_else(|| anyhow!("missing connection"))
    }

    pub fn add_asset(&self, asset: &structures::Asset) -> Result<()> {
        let query = format!(
            "INSERT INTO assets VALUES ({});",
            serde_json::to_string(asset)?
        );

        self.get_connection()?.execute(query)?;

        Ok(())
    }

    pub fn get_assets(&self) -> Result<Vec<structures::Asset>> {
        let mut out = vec![];

        let query = "SELECT * FROM assets";
        let mut statement = self.get_connection()?.prepare(query)?;

        while let Ok(sqlite::State::Row) = statement.next() {
            let data = statement.read::<String, _>("data")?;
            out.push(serde_json::from_str(&data)?);
        }

        Ok(out)
    }
}
