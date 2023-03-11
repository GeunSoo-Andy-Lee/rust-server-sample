use std::{net::SocketAddr, time::Duration, sync::Arc, str::FromStr};

use async_channel::unbounded;
use futures_util::{StreamExt, SinkExt};
use tokio::net::{TcpListener, TcpStream};
//use std::{net::{TcpStream, TcpListener}};
//use tokio::{net::{ToSocketAddrs}};

use tokio_tungstenite::{WebSocketStream, accept_hdr_async};
use tokio_tungstenite::tungstenite::{Message, Result, Error};
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response, ErrorResponse};

use crate::net::{Peer, self};

#[derive(Default)]
pub struct WebSocketListenerBuilder {
}

impl WebSocketListenerBuilder {
    fn new() -> Self {
        Self {
            ..Self::default()
        }
    } 

    pub fn build(self) -> WebSocketListener {
        WebSocketListener {
            context: Arc::new(Context {
                keepalive: Duration::from_millis(5000),
                address: SocketAddr::from_str("0.0.0.0:8000").expect("invalid address"),
            })
        }
    }
}

struct Context {
    keepalive: Duration,
    address: SocketAddr,
}

pub struct WebSocketListener {
    context: Arc<Context>,
}

impl WebSocketListener {

    pub fn builder() -> WebSocketListenerBuilder {
        WebSocketListenerBuilder::new()
    }

    pub async fn start(self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.context.address).await.expect("Can't listen");
        log::info!("Listening on: {:?}", self.context.address);

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream.peer_addr().expect("connected streams should have a peer address");
            log::info!("connected from {}", peer);
            //let _ = handle_accept(peer, stream).await;
            let _ = tokio::spawn(handle_accept(stream, self.context.clone()));
        }

        Ok(())
    }
}

async fn handle_accept(stream: TcpStream, context: Arc<Context>) -> std::io::Result<()> {
    let peer = stream.peer_addr().expect("connected streams should have a peer address");
    log::info!("connected from {}", peer);

    //let local_address = stream.local_addr();
    //let peer_address = stream.peer_addr();

    let mut path = Arc::new("".to_string());
    //let path = RefCell::new("".to_string());
    let headers_callback = {
        //let mut tmp = path;
        |req: &Request, res: Response| -> Result<Response, ErrorResponse> {
            //let header = req.headers().get("Authentication").unwrap();
            path = Arc::new(req.uri().path().to_string());
            Ok(res)
        }
    };

    //let stream = accept_async(stream).await?;
    let stream = accept_hdr_async(stream, headers_callback).await.unwrap();
    //println!("path: {}", path);

    if let Err(e) = handle_receive(peer, stream, context.clone()).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => log::error!("Error processing connection: {}", err),
        }
    }

    Ok(())
}

async fn handle_receive(peer: SocketAddr, stream: WebSocketStream<TcpStream>, context: Arc<Context>) -> Result<()> {

    let (mut write_sock_stream, mut read_sock_stream) = stream.split();

    let mut keepalive = tokio::time::interval(context.keepalive);

    let (tx, rx) = unbounded::<net::Data>();
    let end_point = Peer::new(tx);

    // Echo incoming WebSocket messages and send a message periodically every second.
    loop {
        tokio::select! {
            // 실제 소켓으로 부터 수신 되는 데이터
            data = read_sock_stream.next() => {
                match data.unwrap() {
                    Ok(data) => {
                        match data {
                            Message::Binary(data) => {
                                end_point.send(net::Data::Binary(data.into())).await?
                            },
                            Message::Text(str) => {
                                end_point.send(net::Data::Text(str)).await?
                            },
                            _ => break,
                        }
                    },
                    Err(e) => {
                        log::error!("socket error: {e}");
                        break
                    },
                    //_ => break,
                }
            }

            // EndPoint::send() 호출 시, 수신되는 channel 이벤트로, 실제 소켓에 send 를 함.
            event = rx.recv() => {
                match event {
                    Ok(msg) => {
                        match msg {
                            net::Data::Binary(buf) => {
                                write_sock_stream.send(Message::Binary(buf.to_vec())).await?;
                            },
                            net::Data::Text(buf) => {
                                write_sock_stream.send(Message::Text(buf)).await?;
                            },
                            _ => break,
                        }
                    },
                    Err(e) => {
                        log::error!("endpoint channel error: {e}");
                        break
                    }
                }
            }

            // check keep alive
            _ = keepalive.tick() => {
                //let dummy = [].to_vec();
                //ws_sender.send(Message::Ping(dummy)).await?;
                //ws_sender.send(Message::Text("tick".to_owned())).await?;
            }
        }
    }

    log::info!("disconnect to {}", peer);
    Ok(())
}
