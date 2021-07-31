use std::io;

use tokio::signal::unix::{signal, Signal, SignalKind};

#[derive(Debug, thiserror::Error)]
#[error("signal received")]
pub(crate) struct SignalReceived;

#[derive(Debug)]
pub(crate) struct Sig {
    alrm: Signal,
    hup: Signal,
    int: Signal,
    pipe: Signal,
    quit: Signal,
    term: Signal,
    usr1: Signal,
    usr2: Signal,
}

impl Sig {
    pub(crate) fn new() -> io::Result<Self> {
        Ok(Self {
            alrm: signal(SignalKind::alarm())?,
            hup: signal(SignalKind::hangup())?,
            int: signal(SignalKind::interrupt())?,
            pipe: signal(SignalKind::pipe())?,
            quit: signal(SignalKind::quit())?,
            term: signal(SignalKind::terminate())?,
            usr1: signal(SignalKind::user_defined1())?,
            usr2: signal(SignalKind::user_defined2())?,
        })
    }

    pub(crate) async fn recv(&mut self) -> Result<(), SignalReceived> {
        tokio::select! {
            _ = self.alrm.recv() => Err(SignalReceived),
            _ = self.hup.recv() => Err(SignalReceived),
            _ = self.int.recv() => Err(SignalReceived),
            _ = self.pipe.recv() => Err(SignalReceived),
            _ = self.quit.recv() => Err(SignalReceived),
            _ = self.term.recv() => Err(SignalReceived),
            _ = self.usr1.recv() => Err(SignalReceived),
            _ = self.usr2.recv() => Err(SignalReceived),
        }
    }
}
