use std::io;

use failure::Fail;
use futures::channel::mpsc;

pub use connection::*;
pub use database::*;
pub use listener::*;

mod connection;
mod database;
mod listener;

pub mod model {
    pub use crate::pkt::att::{Handle, Uuid, Uuid128, Uuid16};
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
