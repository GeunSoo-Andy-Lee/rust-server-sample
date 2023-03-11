use std::{io::Error, io::ErrorKind};

use async_channel::Sender;

use crate::net::Data;

#[derive(Clone)]
pub struct Peer
{
    tx: Sender<Data>,   // Sender<SocketEvent>
}

impl Peer {
    pub(crate) fn new(tx: Sender<Data>) -> Self {
        Self {
            tx: tx,
        }
    }

    pub async fn send(&self, data: Data) -> std::io::Result<()>
    {
        match self.tx.send(data).await {
            Err(e) => Err(Error::new(ErrorKind::Other, format!("{e}"))),
            _ => Ok(()),
        }
    }

    pub async fn close(&self) -> std::io::Result<()>
    {
        match self.tx.send(Data::Close).await {
            Err(e) => Err(Error::new(ErrorKind::Other, format!("{e}"))),
            _ => Ok(()),
        }
    }
}
