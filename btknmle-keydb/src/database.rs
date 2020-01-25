use std::any::TypeId;
use std::collections::HashSet;
use std::fs::{OpenOptions, Permissions};
use std::io;
use std::os::unix::fs::PermissionsExt as _;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rusqlite::{Connection, OpenFlags};
use tokio::task;

use btknmle_pkt::CodecError;

#[derive(Debug, failure::Fail)]
pub(crate) enum Error {
    #[fail(display = "{}", _0)]
    Sqlite(rusqlite::Error),
    #[fail(display = "{}", _0)]
    Io(io::Error),
    #[fail(display = "{}", _0)]
    Other(String),
}

impl From<rusqlite::Error> for Error {
    fn from(v: rusqlite::Error) -> Self {
        Self::Sqlite(v)
    }
}

impl From<io::Error> for Error {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}

impl From<CodecError> for Error {
    fn from(v: CodecError) -> Self {
        Self::Other(format!("{}", v))
    }
}

impl From<task::JoinError> for Error {
    fn from(v: task::JoinError) -> Self {
        Self::Other(format!("{}", v))
    }
}

pub(crate) trait DatabaseItem: Sized {
    const DDL: &'static str;

    fn store(&self, connection: &mut Connection) -> Result<(), Error>;
    fn load(connection: &mut Connection) -> Result<Vec<Self>, Error>;
}

#[derive(Debug)]
pub(crate) struct Database {
    inner: Arc<Mutex<Inner>>,
}

impl Database {
    pub(crate) async fn new<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_owned();
        let file = OpenOptions::new().create(true).write(true).open(&path)?;
        file.set_permissions(Permissions::from_mode(0o600))?;
        drop(file);

        let connection = task::spawn_blocking(|| {
            Connection::open_with_flags(
                path,
                OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )
        })
        .await??;
        Self::new_with(connection)
    }

    fn new_with(connection: Connection) -> Result<Self, Error> {
        Ok(Self {
            inner: Arc::new(Mutex::new(Inner::new(connection)?)),
        })
    }

    pub(crate) async fn store<I>(&mut self, item: I) -> Result<(), Error>
    where
        I: DatabaseItem + Send + 'static,
    {
        let inner = self.inner.clone();
        task::spawn_blocking(move || {
            let mut inner = inner.lock().unwrap();
            inner.store(&item)
        })
        .await?
    }

    pub(crate) async fn load<I>(&mut self) -> Result<Vec<I>, Error>
    where
        I: DatabaseItem + Send + 'static,
    {
        let inner = self.inner.clone();
        task::spawn_blocking(move || {
            let mut inner = inner.lock().unwrap();
            inner.load()
        })
        .await?
    }
}

#[derive(Debug)]
struct Inner {
    initialized: HashSet<TypeId>,
    connection: Connection,
}

impl Inner {
    fn new(connection: Connection) -> Result<Self, Error> {
        Ok(Self {
            connection,
            initialized: HashSet::new(),
        })
    }

    fn ensure_init<I>(&mut self) -> Result<(), Error>
    where
        I: DatabaseItem + 'static,
    {
        let type_id = TypeId::of::<I>();
        if self.initialized.insert(type_id) {
            self.connection.execute_batch(I::DDL)?;
        }
        Ok(())
    }

    fn store<I>(&mut self, item: &I) -> Result<(), Error>
    where
        I: DatabaseItem + 'static,
    {
        self.ensure_init::<I>()?;
        item.store(&mut self.connection)
    }

    fn load<I>(&mut self) -> Result<Vec<I>, Error>
    where
        I: DatabaseItem + 'static,
    {
        self.ensure_init::<I>()?;
        I::load(&mut self.connection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use btknmle_pkt::mgmt::{Address, AddressType, IdentityResolvingKey, LongTermKey};
    use std::convert::TryFrom as _;

    #[test]
    fn test_inner() {
        let connection = Connection::open_in_memory().unwrap();
        let mut inner = Inner::new(connection).unwrap();

        let irks = IdentityResolvingKey::new(
            Address::try_from("00:11:22:33:44:55".to_string()).unwrap(),
            AddressType::LeRandom,
            [0; 16],
        );
        inner.store(&irks).unwrap();

        let irks = IdentityResolvingKey::new(
            Address::try_from("00:11:22:33:44:55".to_string()).unwrap(),
            AddressType::LePublic,
            [0; 16],
        );
        inner.store(&irks).unwrap();

        let irks = IdentityResolvingKey::new(
            Address::try_from("99:88:77:66:55:44".to_string()).unwrap(),
            AddressType::LeRandom,
            [0; 16],
        );
        inner.store(&irks).unwrap();

        println!("{:?}", inner.load::<IdentityResolvingKey>());
    }

    #[tokio::test]
    async fn test() {
        let connection = Connection::open_in_memory().unwrap();
        let mut database = Database::new_with(connection).unwrap();

        let irks = IdentityResolvingKey::new(
            Address::try_from("00:11:22:33:44:55".to_string()).unwrap(),
            AddressType::LeRandom,
            [0; 16],
        );
        database.store(irks).await.unwrap();

        let irks = IdentityResolvingKey::new(
            Address::try_from("00:11:22:33:44:55".to_string()).unwrap(),
            AddressType::LePublic,
            [0; 16],
        );
        database.store(irks).await.unwrap();

        let irks = IdentityResolvingKey::new(
            Address::try_from("99:88:77:66:55:44".to_string()).unwrap(),
            AddressType::LeRandom,
            [0; 16],
        );
        database.store(irks).await.unwrap();

        println!("{:?}", database.load::<IdentityResolvingKey>().await);
    }

    #[tokio::test]
    async fn test2() {
        let tmp = mktemp::TempFile::new("", "").unwrap();
        let mut database = Database::new(tmp.path().to_string()).await.unwrap();

        let irks = IdentityResolvingKey::new(
            Address::try_from("00:11:22:33:44:55".to_string()).unwrap(),
            AddressType::LeRandom,
            [0; 16],
        );
        database.store(irks).await.unwrap();

        let irks = IdentityResolvingKey::new(
            Address::try_from("00:11:22:33:44:55".to_string()).unwrap(),
            AddressType::LePublic,
            [0; 16],
        );
        database.store(irks).await.unwrap();

        let irks = IdentityResolvingKey::new(
            Address::try_from("99:88:77:66:55:44".to_string()).unwrap(),
            AddressType::LeRandom,
            [0; 16],
        );
        database.store(irks).await.unwrap();

        let ltks = LongTermKey::new(
            Address::try_from("99:88:77:66:55:44".to_string()).unwrap(),
            AddressType::LeRandom,
            0,
            0,
            0,
            [0; 2],
            [0; 8],
            [0; 16],
        );
        database.store(ltks).await.unwrap();

        println!("{:?}", database.load::<IdentityResolvingKey>().await);
        println!("{:?}", database.load::<LongTermKey>().await);
    }
}
