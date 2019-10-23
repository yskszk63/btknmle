use std::io;

use futures::channel::mpsc;
use failure::Fail;

pub use database::*;
pub use listener::*;
pub use connection::*;

mod database;
mod listener;
mod connection;

pub mod model {
    pub use crate::pkt::att::{Handle, Uuid, Uuid16, Uuid128};
}

#[derive(Debug, Fail)]
pub enum GattError {
    #[fail(display = "IO Error occurred {}", _0)]
    Io(#[fail(cause)] io::Error),

    #[fail(display = "Send Error occurred {}", _0)]
    Send(#[fail(cause)] mpsc::SendError),
}

impl From<io::Error> for GattError {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}

impl From<mpsc::SendError> for GattError {
    fn from(v: mpsc::SendError) -> Self {
        Self::Send(v)
    }
}

pub type Result<T> = std::result::Result<T, GattError>;
