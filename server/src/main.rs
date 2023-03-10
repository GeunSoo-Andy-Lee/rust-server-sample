use futures_util::{SinkExt, StreamExt};
use std::{net::SocketAddr, time::Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::{Error}};
use tokio_tungstenite::tungstenite::{Message, Result};

mod tnmt;
use tnmt::net::{self};

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => log::error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    log::info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(1000));

    // Echo incoming WebSocket messages and send a message periodically every second.

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() ||msg.is_binary() {
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                ws_sender.send(Message::Text("tick".to_owned())).await?;
            }
        }
    }

    Ok(())
}

struct MyServer;


#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    //log::info!("Listening on: {}", addr);

    let builder = net::WebSocketListener::builder();
    let server = builder.build();
    server.start().await.unwrap();

    //while let Ok((stream, _)) = listener.accept().await {
    //    let peer = stream.peer_addr().expect("connected streams should have a peer address");
    //    log::info!("Peer address: {}", peer);
    //    //server.accept(peer, stream);

    //    //tokio::spawn(accept_connection(peer, stream));
    //}
}
