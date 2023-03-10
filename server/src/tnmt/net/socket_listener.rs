use std::{net::SocketAddr, time::Duration};

use futures_util::stream::{SplitSink, SplitStream};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

trait ServerBuilder {
    fn keepalive(timeout: usize);
}

pub struct SocketListenerBuilder {
}

impl SocketListenerBuilder {
    pub fn new() -> Self {
        Self {}
    } 
    pub fn build(self) -> SocketListener {
        SocketListener {}
    }
}

pub struct SocketListener {
}

impl SocketListener {
    pub fn builder() -> SocketListenerBuilder {
        SocketListenerBuilder::new()
    }

    pub async fn start(self) -> std::io::Result<()> {
        let addr = "0.0.0.0:8080";
        self.internal_accept(addr).await
    }

    async fn internal_accept(self, addr: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(&addr).await.expect("Can't listen");
        log::info!("Listening on: {:?}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            tokio::task::spawn_local(handle_accept(stream));
            //tokio::spawn(handle_accept(stream));
        }

        Ok(())
    }
}

async fn handle_accept(stream: TcpStream) -> std::io::Result<()> {
    let peer = stream.peer_addr().expect("connected streams should have a peer address");
    log::info!("connected from {}", peer);
    Ok(handle_receive(stream).await.unwrap())
}

async fn handle_receive(mut stream: TcpStream) -> std::io::Result<()> {
    let local_address = stream.local_addr()?;
    let peer_address = stream.peer_addr()?;

    let (mut read_stream, mut write_stream) = stream.split();
    let mut keepalive = tokio::time::interval(Duration::from_millis(1000));

    let mut buf = vec![0u8; 1024];
    loop {
        tokio::select! {
            n = read_stream.read(&mut buf) => {
                let nbytes = n?;
                if 0 == nbytes {
                    break;
                }

                write_stream.write_all(&buf[0..nbytes])
                .await
                .expect("failed to write data to socket");
            }

            _ = keepalive.tick() => {
                //let dummy = [].to_vec();
                //ws_sender.send(Message::Ping(dummy)).await?;
                //ws_sender.send(Message::Text("tick".to_owned())).await?;
            }
        }
    }

    log::info!("disconnect to {}", peer_address);
    Ok(())
}
