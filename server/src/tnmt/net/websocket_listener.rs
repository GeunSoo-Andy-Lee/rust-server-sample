use std::{net::SocketAddr, time::Duration};

use async_std::channel::Receiver;
use futures_util::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}, Stream};
//use tokio::{net::{TcpStream, TcpListener}, task::JoinHandle, io::AsyncWriteExt};
use tokio::{task::JoinHandle, io::AsyncWriteExt, net::{TcpListener, TcpStream}};
//use std::{net::{TcpStream, TcpListener}};
//use tokio::{net::{ToSocketAddrs}};

use tokio_tungstenite::{WebSocketStream, accept_async, accept_hdr_async};
use tokio_tungstenite::tungstenite::{Message, Result, Error};
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response, ErrorResponse};

pub struct WebSocketListenerBuilder {
}

impl WebSocketListenerBuilder {
    fn new() -> Self {
        Self {}
    } 
    pub fn build(self) -> WebSocketListener {
        WebSocketListener {}
    }
}

pub struct WebSocketListener {
    //server: Box<dyn IServer>,
}

impl WebSocketListener {

    pub fn builder() -> WebSocketListenerBuilder {
        WebSocketListenerBuilder::new()
    }

    pub async fn start(self) -> std::io::Result<()> {
        let addr = "0.0.0.0:8080";
        self.internal_accept(addr).await
    }

    async fn internal_accept(self, addr: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(&addr).await.expect("Can't listen");
        log::info!("Listening on: {:?}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream.peer_addr().expect("connected streams should have a peer address");
            tokio::spawn(handle_accept(peer, stream));
        }

        Ok(())
    }
}

async fn handle_accept(peer: SocketAddr, stream: TcpStream) -> std::io::Result<()> {
    log::info!("connected from {}", peer);

    if let Err(e) = handle_receive(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => log::error!("Error processing connection: {}", err),
        }
    }

    Ok(())
}

//type StreamResult = Result<Message, tokio_tungstenite::tungstenite::Error>;
//SplitSink<WebSocketStream<tokio::net::TcpStream>, Result<Message, tokio_tungstenite::tungstenite::Error>>`

async fn handle_receive(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let local_address = stream.local_addr();
    let peer_address = stream.peer_addr();

    let path = std::cell::RefCell::new("".to_string());
    let headers_callback = |req: &Request, res: Response| -> Result<Response, ErrorResponse> {
        println!("{:?}", req.uri());
        path.replace(req.uri().path().to_string());
        Ok(res)
    };

    let websocket_stream = accept_async(stream).await?;
    //let websocket_stream = accept_hdr_async(stream, headers_callback).await?;
    //println!("path: {}", path.borrow());

    let (mut ws_send, mut ws_recv) = websocket_stream.split();

    let mut interval = tokio::time::interval(Duration::from_millis(1000));

    //let socket = SocketImpl { read_stream: ws_recv, write_stream: ws_send };
    // Echo incoming WebSocket messages and send a message periodically every second.

    loop {
        tokio::select! {
            msg = ws_recv.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() ||msg.is_binary() {
                            ws_send.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }

            _ = interval.tick() => {
                //let dummy = [].to_vec();
                //ws_sender.send(Message::Ping(dummy)).await?;
                //ws_sender.send(Message::Text("tick".to_owned())).await?;
            }
        }
    }

    log::info!("disconnect to {}", peer);
    Ok(())
}



struct WebSocket {
    tx: SplitSink<WebSocketStream<TcpStream>, Message>,
    //rx: SplitStream<WebSocketStream<TcpStream>>,
}

impl WebSocket {
    async fn new(stream: TcpStream) -> std::io::Result<(Self, SplitStream<WebSocketStream<TcpStream>>)> {
        let (tx, rx) = match accept_async(stream).await {
            Ok(stream) => Ok(stream),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to accept: {e}"))),
        }?.split();

        //Ok(Self { tx, rx })
        Ok((Self { tx }, rx))
    }

    pub async fn close(&mut self) {
        self.tx.close().await;
    }

    pub async fn send(&mut self, data: &[u8]) -> std::io::Result<()> {
        match self.tx.send(Message::Binary(data.to_vec())).await {
            Ok(_) => Ok(()),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{e}"))),
        }
    }
}
