use std::io;

use btknmle_pkt::mgmt::IdentityResolvingKey;
use btknmle_pkt::mgmt::LongTermKey;

#[async_trait::async_trait]
pub trait KeyStore {
    async fn load_local_irk(&mut self) -> io::Result<[u8; 16]>;

    async fn load_irks(&mut self) -> io::Result<Vec<IdentityResolvingKey>>;

    async fn load_ltks(&mut self) -> io::Result<Vec<LongTermKey>>;

    async fn store_irks(&mut self, key: IdentityResolvingKey) -> io::Result<()>;

    async fn store_ltks(&mut self, key: LongTermKey) -> io::Result<()>;
}
