use std::io;

use futures::channel::mpsc;
use thiserror::Error;

pub use connection::*;
pub use database::*;
pub use listener::*;

mod connection;
mod database;
mod listener;

pub mod model {
    pub use crate::pkt::att::Handle;
    pub use crate::pkt::{Uuid, Uuid128, Uuid16};
}

#[derive(Error, Debug)]
pub enum GattError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Send(#[from] mpsc::SendError),
}

pub type Result<T> = std::result::Result<T, GattError>;
