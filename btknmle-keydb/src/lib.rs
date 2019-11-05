use std::convert::TryInto as _;
use std::fs::Permissions;
use std::io;
use std::num::ParseIntError;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use bytes::Bytes;
use futures::TryStreamExt as _;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use toml::value::{Table, Value};

use btknmle_pkt::mgmt::{IdentityResolvingKey, LongTermKey};

fn to_hex(buf: &[u8]) -> String {
    buf.iter().map(|v| format!("{:02X}", v)).collect()
}

fn from_hex(s: &str) -> Result<Bytes, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

async fn ensure_dir(path: impl AsRef<Path>) -> io::Result<()> {
    let metadata = match fs::metadata(&path).await {
        Ok(metadata) => Ok(metadata),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            fs::create_dir_all(&path).await?;
            fs::set_permissions(&path, Permissions::from_mode(0o700)).await?;
            fs::metadata(&path).await
        }
        Err(e) => Err(e),
    }?;

    if !metadata.is_dir() || (metadata.permissions().mode() & 0o777) != 0o700 {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "{:?} is not a directory or invalid permissions",
                path.as_ref()
            ),
        ))
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct KeyDb {
    path: PathBuf,
}

impl KeyDb {
    pub async fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();

        ensure_dir(&path).await?;
        ensure_dir(path.join("irks")).await?;
        ensure_dir(path.join("ltks")).await?;

        Ok(KeyDb { path: path.into() })
    }

    async fn load(&mut self, name: &str) -> io::Result<Value> {
        let content = fs::read_dir(self.path.join(name))
            .await?
            .and_then(|f| fs::read(f.path()))
            .try_concat()
            .await?;
        let content = String::from_utf8_lossy(&content).to_owned();
        let content =
            toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let content = Value::Table(content);
        Ok(content)
    }

    pub async fn load_irks(&mut self) -> io::Result<Vec<IdentityResolvingKey>> {
        let toml = self.load("irks").await?;
        let elements = match toml.get("irks").and_then(Value::as_array) {
            Some(e) => e,
            None => return Ok(vec![]),
        };
        let result = elements
            .iter()
            .map(|entry| {
                let address = entry
                    .get("address")
                    .and_then(Value::as_str)
                    .ok_or_else(|| io::ErrorKind::InvalidData)?
                    .to_string()
                    .try_into()
                    .map_err(|_| io::ErrorKind::InvalidData)?;

                let address_type = entry
                    .get("address_type")
                    .and_then(Value::as_integer)
                    .ok_or_else(|| io::ErrorKind::InvalidData)?
                    as u8;
                let address_type = address_type
                    .try_into()
                    .map_err(|_| io::ErrorKind::InvalidData)?;

                let value = entry
                    .get("value")
                    .and_then(Value::as_str)
                    .ok_or_else(|| io::ErrorKind::InvalidData)?;
                let v = from_hex(&value).map_err(|_| io::ErrorKind::InvalidData)?;
                let mut value = [0; 16];
                value.copy_from_slice(&v);

                Ok(IdentityResolvingKey::new(address, address_type, value))
            })
            .collect::<io::Result<_>>()?;
        Ok(result)
    }

    pub async fn load_ltks(&mut self) -> io::Result<Vec<LongTermKey>> {
        let toml = self.load("ltks").await?;
        let elements = match toml.get("ltks").and_then(|e| e.as_array()) {
            Some(e) => e,
            None => return Ok(vec![]),
        };
        let result = elements
            .iter()
            .map(|entry| {
                let address = entry
                    .get("address")
                    .and_then(Value::as_str)
                    .ok_or_else(|| io::ErrorKind::InvalidData)?
                    .to_string()
                    .try_into()
                    .map_err(|_| io::ErrorKind::InvalidData)?;

                let address_type = entry
                    .get("address_type")
                    .and_then(Value::as_integer)
                    .ok_or_else(|| io::ErrorKind::InvalidData)?
                    as u8;
                let address_type = address_type
                    .try_into()
                    .map_err(|_| io::ErrorKind::InvalidData)?;

                let key_type = entry
                    .get("key_type")
                    .and_then(Value::as_integer)
                    .ok_or_else(|| io::ErrorKind::InvalidData)?
                    as u8;

                let master = entry
                    .get("master")
                    .and_then(Value::as_integer)
                    .ok_or_else(|| io::ErrorKind::InvalidData)? as u8;

                let encryption_size = entry
                    .get("encryption_size")
                    .and_then(Value::as_integer)
                    .ok_or_else(|| io::ErrorKind::InvalidData)?
                    as u8;

                let encryption_diversifier = entry
                    .get("encryption_diversifier")
                    .and_then(Value::as_str)
                    .ok_or_else(|| io::ErrorKind::InvalidData)?;
                let v =
                    from_hex(&encryption_diversifier).map_err(|_| io::ErrorKind::InvalidData)?;
                let mut encryption_diversifier = [0; 2];
                encryption_diversifier.copy_from_slice(&v);

                let random_number = entry
                    .get("random_number")
                    .and_then(Value::as_str)
                    .ok_or_else(|| io::ErrorKind::InvalidData)?;
                let v = from_hex(&random_number).map_err(|_| io::ErrorKind::InvalidData)?;
                let mut random_number = [0; 8];
                random_number.copy_from_slice(&v);

                let value = entry
                    .get("value")
                    .and_then(Value::as_str)
                    .ok_or_else(|| io::ErrorKind::InvalidData)?;
                let v = from_hex(&value).map_err(|_| io::ErrorKind::InvalidData)?;
                let mut value = [0; 16];
                value.copy_from_slice(&v);

                Ok(LongTermKey::new(
                    address,
                    address_type,
                    key_type,
                    master,
                    encryption_size,
                    encryption_diversifier,
                    random_number,
                    value,
                ))
            })
            .collect::<io::Result<_>>()?;
        Ok(result)
    }

    pub async fn store_irks(&mut self, key: &IdentityResolvingKey) -> io::Result<()> {
        let mut entry = Table::new();
        entry.insert("address".into(), key.address().to_string().into());
        entry.insert("address_type".into(), u8::from(key.address_type()).into());
        entry.insert("value".into(), to_hex(&key.value()).into());

        let mut outer = Table::new();
        outer.insert("irks".into(), Value::Array(vec![Value::Table(entry)]));
        let outer = Value::Table(outer);

        self.write(&outer, "irks", &key.address().to_string()).await
    }

    pub async fn store_ltks(&mut self, key: &LongTermKey) -> io::Result<()> {
        let mut entry = Table::new();
        entry.insert("address".into(), key.address().to_string().into());
        entry.insert("address_type".into(), u8::from(key.address_type()).into());
        entry.insert("key_type".into(), u8::from(key.key_type()).into()); // TODO P256 key
        entry.insert("master".into(), u8::from(key.master()).into());
        entry.insert(
            "encryption_size".into(),
            u8::from(key.encryption_size()).into(),
        );
        entry.insert(
            "encryption_diversifier".into(),
            to_hex(&key.encryption_diversifier()).into(),
        );
        entry.insert("random_number".into(), to_hex(&key.random_number()).into());
        entry.insert("value".into(), to_hex(&key.value()).into());

        let mut outer = Table::new();
        outer.insert("ltks".into(), Value::Array(vec![Value::Table(entry)]));
        let outer = Value::Table(outer);

        self.write(&outer, "ltks", &key.address().to_string()).await
    }

    async fn write(&mut self, v: &Value, name: &str, addr: &str) -> io::Result<()> {
        let path = self.path.join(name).join(addr);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .await?;
        file.write(v.to_string().as_bytes()).await?;
        file.set_permissions(Permissions::from_mode(0o600)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use btknmle_pkt::Codec as _;
    use bytes::IntoBuf;

    #[tokio::test]
    async fn it_works() {
        let db = "/tmp/add24f28-f506-4b72-8262-cd4180bb6b4a";
        fs::remove_dir_all(db).await.ok();
        let mut db = KeyDb::new(db).await.unwrap();

        let r = db.load_irks().await.unwrap();
        println!("{:?}", r);

        let r = db.load_ltks().await.unwrap();
        println!("{:?}", r);

        let k = vec![
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x02, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD,
            0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10,
        ];
        let k = IdentityResolvingKey::parse(&mut k.into_buf()).unwrap();
        db.store_irks(&k).await.unwrap();

        let k = vec![
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x02, 0x01, 0x00, 0x16, 0x11, 0x22, 0x01, 0x23,
            0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
            0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10,
        ];
        let k = LongTermKey::parse(&mut k.into_buf()).unwrap();
        db.store_ltks(&k).await.unwrap();

        let r = db.load_irks().await.unwrap();
        println!("{:?}", r);

        let r = db.load_ltks().await.unwrap();
        println!("{:?}", r);
    }
}
