

#[tokio::main]
async fn main() {
    //let addr = "0.0.0.0:8080";
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    //log::info!("Listening on: {}", addr);
    let builder = base::net::WebSocketListener::builder();
    let server = builder.build();

    let _x: base::net::Peer;
    let _x: base::net::Data;

    server.start().await.unwrap();

    //while let Ok((stream, _)) = listener.accept().await {
    //    let peer = stream.peer_addr().expect("connected streams should have a peer address");
    //    log::info!("Peer address: {}", peer);
    //    //server.accept(peer, stream);

    //    //tokio::spawn(accept_connection(peer, stream));
    //}
}
