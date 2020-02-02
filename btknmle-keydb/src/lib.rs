#![warn(clippy::all)]

use std::io;
use std::path::Path;

use btknmle_pkt::mgmt::{IdentityResolvingKey, LongTermKey};

mod database;
mod model;

#[derive(Debug)]
pub struct KeyDb {
    database: database::Database,
}

impl KeyDb {
    pub async fn new(path: impl AsRef<Path> + Send + 'static) -> io::Result<Self> {
        let database = database::Database::new(path)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(Self { database })
    }

    pub async fn load_local_irk(&mut self) -> io::Result<[u8; 16]> {
        let result = self
            .database
            .load::<model::LocalIdentityResolvingKey>()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(result.into_iter().next().unwrap().into())
    }

    pub async fn load_irks(&mut self) -> io::Result<Vec<IdentityResolvingKey>> {
        let result = self
            .database
            .load::<IdentityResolvingKey>()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(result)
    }

    pub async fn load_ltks(&mut self) -> io::Result<Vec<LongTermKey>> {
        let result = self
            .database
            .load::<LongTermKey>()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(result)
    }

    pub async fn store_irks(&mut self, key: IdentityResolvingKey) -> io::Result<()> {
        self.database
            .store(key)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(())
    }

    pub async fn store_ltks(&mut self, key: LongTermKey) -> io::Result<()> {
        self.database
            .store(key)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(())
    }
}
