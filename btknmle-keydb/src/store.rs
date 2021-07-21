use std::collections::VecDeque;
use std::mem;
use std::path::Path;

use btmgmt::packet::{IdentityResolvingKey, LongTermKey};
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncSeekExt, AsyncWriteExt, BufStream, SeekFrom};

use crate::serde::Wrapper;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Data {
    key_for_resolvable_private_address: Wrapper<[u8; 16]>,
    #[serde(default, skip_serializing_if = "VecDeque::is_empty")]
    irks: VecDeque<Wrapper<IdentityResolvingKey>>,
    #[serde(default, skip_serializing_if = "VecDeque::is_empty")]
    ltks: VecDeque<Wrapper<LongTermKey>>,
}

impl Data {
    fn new() -> Self {
        Self {
            key_for_resolvable_private_address: rand::random::<[u8; 16]>().into(),
            irks: Default::default(),
            ltks: Default::default(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    Serialize(#[from] toml::ser::Error),

    #[error(transparent)]
    Deserialize(#[from] toml::de::Error),
}

#[derive(Debug)]
pub struct Store {
    file: BufStream<File>,
    data: Data,
}

impl Store {
    pub async fn open<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .open(path)
            .await?;
        let mut file = BufStream::new(file);
        let mut buf = vec![];
        io::copy(&mut file, &mut buf).await?;

        if buf.is_empty() {
            let data = Data::new();
            let mut result = Self { file, data };
            result.dump().await?;
            Ok(result)
        } else {
            let data = toml::from_slice(&buf)?;
            Ok(Self { file, data })
        }
    }

    async fn dump(&mut self) -> Result<(), Error> {
        let buf = toml::to_vec(&self.data)?;

        let file = &mut self.file;
        file.get_mut().set_len(0).await?;
        file.get_mut().seek(SeekFrom::Start(0)).await?;
        file.write_all(&buf).await?;
        file.flush().await?;
        Ok(())
    }

    pub fn key_for_resolvable_private_address(&self) -> &[u8; 16] {
        &self.data.key_for_resolvable_private_address.as_ref()
    }

    pub async fn add_irk(&mut self, record: IdentityResolvingKey) -> Result<(), Error> {
        let mut new = self
            .data
            .irks
            .drain(..)
            .filter(|k| k.as_ref().address().as_ref() != record.address().as_ref())
            .collect::<VecDeque<_>>();
        new.push_front(record.into());
        mem::swap(&mut self.data.irks, &mut new);
        self.dump().await?;
        Ok(())
    }

    pub async fn add_ltk(&mut self, record: LongTermKey) -> Result<(), Error> {
        let mut new = self
            .data
            .ltks
            .drain(..)
            .filter(|k| k.as_ref().address().as_ref() != record.address().as_ref())
            .collect::<VecDeque<_>>();
        new.push_front(record.into());
        mem::swap(&mut self.data.ltks, &mut new);
        self.dump().await?;
        Ok(())
    }

    pub fn iter_irks(&self) -> impl Iterator<Item = &'_ IdentityResolvingKey> {
        self.data.irks.iter().map(AsRef::as_ref)
    }

    pub fn iter_ltks(&self) -> impl Iterator<Item = &'_ LongTermKey> {
        self.data.ltks.iter().map(AsRef::as_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use btmgmt::packet::{AddressType, IdentityResolvingKey, LongTermKeyBuilder, LongTermKeyType};

    #[tokio::test]
    async fn test() {
        let tmp = mktemp::TempFile::new("", "").unwrap();
        let mut store = Store::open(tmp.path()).await.unwrap();

        let k = store.key_for_resolvable_private_address().to_vec();
        let v1 = rand::random();
        let v2 = rand::random();
        let v3 = rand::random();

        store
            .add_irk(IdentityResolvingKey::new(
                "00:11:22:33:44:55".parse().unwrap(),
                AddressType::LePublic,
                v1,
            ))
            .await
            .unwrap();
        store
            .add_irk(IdentityResolvingKey::new(
                "55:44:33:22:11:00".parse().unwrap(),
                AddressType::LeRandom,
                v2,
            ))
            .await
            .unwrap();

        store
            .add_ltk(
                LongTermKeyBuilder::default()
                    .address("00:11:22:33:44:55".parse().unwrap())
                    .address_type(AddressType::LeRandom)
                    .key_type(LongTermKeyType::AuthenticatedKey)
                    .master(true)
                    .encryption_size(32)
                    .encryption_diversifier(123)
                    .random_number(v3)
                    .value(v1)
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();

        drop(store);

        let store = Store::open(tmp.path()).await.unwrap();
        assert_eq!(k, store.key_for_resolvable_private_address());
        for (n, irk) in store.iter_irks().enumerate() {
            match n {
                0 => {
                    assert_eq!(&irk.address().to_string(), "55:44:33:22:11:00");
                    assert_eq!(irk.address_type(), &AddressType::LeRandom);
                    assert_eq!(irk.value(), &v2);
                }
                1 => {
                    assert_eq!(&irk.address().to_string(), "00:11:22:33:44:55");
                    assert_eq!(irk.value(), &v1);
                }
                _ => panic!(),
            }
        }

        for ltk in store.iter_ltks() {
            assert_eq!(&ltk.address().to_string(), "00:11:22:33:44:55");
            assert_eq!(ltk.address_type(), &AddressType::LeRandom);
            //assert_eq!(ltk.key_type(), LongTermKeyType::AuthenticatedKey);
            assert_eq!(ltk.master(), &true);
            assert_eq!(ltk.encryption_size(), &32);
            assert_eq!(ltk.encryption_diversifier(), &123);
            assert_eq!(ltk.random_number(), &v3);
            assert_eq!(ltk.value(), &v1);
        }
    }
}
