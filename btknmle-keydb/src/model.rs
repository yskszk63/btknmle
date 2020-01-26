use rusqlite::types::Value;
use rusqlite::Connection;

use crate::database::{DatabaseItem, Error};
use btknmle_pkt::mgmt::{Address, AddressType, IdentityResolvingKey, LongTermKey};
use btknmle_pkt::Codec as _;

#[derive(Debug)]
pub(crate) struct LocalIdentityResolvingKey([u8; 16]);

impl DatabaseItem for LocalIdentityResolvingKey {
    const DDL: &'static str = r#"
        CREATE TABLE IF NOT EXISTS local_identity_resolving_key(
            key INT NOT NULL,
            value BLOB NOT NULL,
            PRIMARY KEY(key)
        );
        INSERT INTO local_identity_resolving_key (key, value) VALUES (0, RANDOMBLOB(16)) ON CONFLICT DO NOTHING;
    "#;

    fn store(&self, _connection: &mut Connection) -> Result<(), Error> {
        Err(Error::Other("NOT SUPPORTED".into()))
    }

    fn load(connection: &mut Connection) -> Result<Vec<Self>, Error> {
        let mut statement = connection.prepare("SELECT value FROM local_identity_resolving_key")?;
        let rows = statement.query_map(rusqlite::NO_PARAMS, |row| {
            let mut value = [0; 16];
            value.copy_from_slice(&row.get::<_, Vec<_>>(0)?);
            Ok(Self(value))
        });
        Ok(rows.map(|r| r.flatten().collect())?)
    }
}

impl From<LocalIdentityResolvingKey> for [u8; 16] {
    fn from(v: LocalIdentityResolvingKey) -> Self {
        v.0
    }
}

impl DatabaseItem for IdentityResolvingKey {
    const DDL: &'static str = r#"
        CREATE TABLE IF NOT EXISTS irks(
            address BLOB NOT NULL,
            address_type INT NOT NULL,
            value BLOB NOT NULL,
            PRIMARY KEY(address, address_type)
        )
    "#;

    fn store(&self, connection: &mut Connection) -> Result<(), Error> {
        let address = Vec::from(self.address().as_ref());
        let address = Value::Blob(address);

        let address_type = match self.address_type() {
            AddressType::BrEdr => 0,
            AddressType::LePublic => 1,
            AddressType::LeRandom => 2,
        };
        let address_type = Value::Integer(address_type);

        let value = Vec::from(self.value().as_ref());
        let value = Value::Blob(value);

        connection.execute(
            r#"
            INSERT INTO irks(address, address_type, value)
                VALUES (?, ?, ?)
            ON CONFLICT(address, address_type) DO
                UPDATE SET value=excluded.value"#,
            &[address, address_type, value],
        )?;
        Ok(())
    }

    fn load(connection: &mut Connection) -> Result<Vec<Self>, Error> {
        let mut statement = connection.prepare("SELECT address, address_type, value FROM irks")?;
        let rows = statement.query_map(rusqlite::NO_PARAMS, |row| {
            let address = Address::parse(&mut row.get::<_, Vec<_>>(0)?.as_ref()).unwrap();
            let address_type = match row.get(1)? {
                0 => AddressType::BrEdr,
                1 => AddressType::LePublic,
                2 => AddressType::LeRandom,
                _ => return Err(rusqlite::Error::InvalidQuery),
            };
            let mut value = [0; 16];
            value.copy_from_slice(&row.get::<_, Vec<_>>(2)?);
            Ok(Self::new(address, address_type, value))
        });
        Ok(rows.map(|r| r.flatten().collect())?)
    }
}

impl DatabaseItem for LongTermKey {
    const DDL: &'static str = r#"
        CREATE TABLE IF NOT EXISTS ltks(
            address BLOB NOT NULL,
            address_type INT NOT NULL,
            key_type INT NOT NULL,
            master INT NOT NULL,
            encryption_size INT NOT NULL,
            encryption_diversifier INT NOT NULL,
            random_number INT NOT NULL,
            value BLOB NOT NULL,
            PRIMARY KEY(address, address_type, key_type, master)
        )
    "#;

    fn store(&self, connection: &mut Connection) -> Result<(), Error> {
        let address = Vec::from(self.address().as_ref());
        let address = Value::Blob(address);

        let address_type = match self.address_type() {
            AddressType::BrEdr => 0,
            AddressType::LePublic => 1,
            AddressType::LeRandom => 2,
        };
        let address_type = Value::Integer(address_type);

        let key_type = Value::Integer(self.key_type().into());

        let master = Value::Integer(self.master().into());

        let encryption_size = Value::Integer(self.encryption_size().into());

        let encryption_diversifier = Vec::from(self.encryption_diversifier().as_ref());
        let encryption_diversifier = Value::Blob(encryption_diversifier);

        let random_number = Vec::from(self.random_number().as_ref());
        let random_number = Value::Blob(random_number);

        let value = Vec::from(self.value().as_ref());
        let value = Value::Blob(value);

        connection.execute(r#"
            INSERT INTO ltks(
                    address, address_type, key_type, master, encryption_size, encryption_diversifier, random_number, value)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(address, address_type, key_type, master) DO
                UPDATE SET
                     encryption_size=excluded.encryption_size
                    ,encryption_diversifier=excluded.encryption_diversifier
                    ,random_number=excluded.random_number
                    ,value=excluded.value
                    "#,
            &[address, address_type, key_type, master, encryption_size, encryption_diversifier, random_number, value])?;
        Ok(())
    }

    fn load(connection: &mut Connection) -> Result<Vec<Self>, Error> {
        let mut statement = connection.prepare(
            "SELECT address, address_type, key_type, master, encryption_size, encryption_diversifier, random_number, value FROM ltks")?;
        let rows = statement.query_map(rusqlite::NO_PARAMS, |row| {
            let address = Address::parse(&mut row.get::<_, Vec<_>>(0)?.as_ref()).unwrap();
            let address_type = match row.get(1)? {
                0 => AddressType::BrEdr,
                1 => AddressType::LePublic,
                2 => AddressType::LeRandom,
                _ => return Err(rusqlite::Error::InvalidQuery),
            };
            let key_type = row.get(2)?;
            let master = row.get(3)?;
            let encryption_size = row.get(4)?;

            let mut encryption_diversifier = [0; 2];
            encryption_diversifier.copy_from_slice(&row.get::<_, Vec<_>>(5)?);

            let mut random_number = [0; 8];
            random_number.copy_from_slice(&row.get::<_, Vec<_>>(6)?);

            let mut value = [0; 16];
            value.copy_from_slice(&row.get::<_, Vec<_>>(7)?);

            Ok(Self::new(
                address,
                address_type,
                key_type,
                master,
                encryption_size,
                encryption_diversifier,
                random_number,
                value,
            ))
        });
        Ok(rows.map(|r| r.flatten().collect())?)
    }
}
