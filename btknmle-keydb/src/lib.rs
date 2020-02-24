#![warn(clippy::all)]

use std::io;
use std::path::Path;

use btknmle_pkt::mgmt::{IdentityResolvingKey, LongTermKey};
use btknmle_server::KeyStore;

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
}

#[async_trait::async_trait]
impl KeyStore for KeyDb {
    async fn load_local_irk(&mut self) -> io::Result<[u8; 16]> {
        let result = self
            .database
            .load::<model::LocalIdentityResolvingKey>()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(result.into_iter().next().unwrap().into())
    }

    async fn load_irks(&mut self) -> io::Result<Vec<IdentityResolvingKey>> {
        let result = self
            .database
            .load::<IdentityResolvingKey>()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(result)
    }

    async fn load_ltks(&mut self) -> io::Result<Vec<LongTermKey>> {
        let result = self
            .database
            .load::<LongTermKey>()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(result)
    }

    async fn store_irks(&mut self, key: IdentityResolvingKey) -> io::Result<()> {
        self.database
            .store(key)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(())
    }

    async fn store_ltks(&mut self, key: LongTermKey) -> io::Result<()> {
        self.database
            .store(key)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[tokio::test]
    async fn test_open() {
        KeyDb::new("/tmp/a21ef215-25da-4c51-ba26-2c54faff67f5")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_local_irks() {
        let f = Path::new("/tmp/50f716bf-7254-4211-b460-be7fa17ddaa1");
        fs::remove_file(f).ok();
        let mut db = KeyDb::new(f).await.unwrap();
        db.load_local_irk().await.unwrap();
    }

    #[tokio::test]
    async fn test_irks() {
        let f = Path::new("/tmp/d17aba67-3aac-4f70-b67f-e85a238f79ee");
        fs::remove_file(f).ok();
        let mut db = KeyDb::new(f).await.unwrap();
        let address = "00:00:00:00:00:00"
            .parse::<btknmle_pkt::mgmt::Address>()
            .unwrap();
        let address_type = btknmle_pkt::mgmt::AddressType::LePublic;
        let value = [0; 16];
        db.store_irks(IdentityResolvingKey::new(
            address.clone(),
            address_type.clone(),
            value,
        ))
        .await
        .unwrap();
        let results = db.load_irks().await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0],
            IdentityResolvingKey::new(address, address_type, value)
        );
    }

    #[tokio::test]
    async fn test_ltks() {
        let f = Path::new("/tmp/75a91193-70f0-4a34-971c-2cd6cdfba8c0");
        fs::remove_file(f).ok();
        let mut db = KeyDb::new(f).await.unwrap();
        let address = "00:00:00:00:00:00"
            .parse::<btknmle_pkt::mgmt::Address>()
            .unwrap();
        let address_type = btknmle_pkt::mgmt::AddressType::LePublic;
        let key_type = 0;
        let master = 0;
        let encryption_size = 0;
        let encryption_diversifier = [0; 2];
        let random_number = [0; 8];
        let value = [0; 16];
        db.store_ltks(LongTermKey::new(
            address.clone(),
            address_type.clone(),
            key_type,
            master,
            encryption_size,
            encryption_diversifier,
            random_number,
            value,
        ))
        .await
        .unwrap();
        let results = db.load_ltks().await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0],
            LongTermKey::new(
                address.clone(),
                address_type.clone(),
                key_type,
                master,
                encryption_size,
                encryption_diversifier,
                random_number,
                value
            )
        );
    }
}
