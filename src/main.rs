use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket};
use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a broadcast channel to send messages to all connected clients
    let (tx, _) = broadcast::channel::<Message>(10);

    // Clone the sender for use in the closure
    // Clone the sender for use in the closure
    let tx_clone = tx.clone();

    // WebSocket route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = tx_clone.clone();
            ws.on_upgrade(move |socket| {
                let tx = tx.clone();
                async move { handle_connection(socket, tx).await }
            })
        });

    // server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    // server address
    println!("WebSocket server started at: ws://{}", addr);

    // Starting WebSocket server
    warp::serve(ws_route).run(addr).await;
}

async fn handle_connection(ws: WebSocket, tx: broadcast::Sender<Message>) {
    // Split WebSocket into sender and receiver streams
    let (mut ws_tx, mut ws_rx) = ws.split();

    // Subscribe the receiver to the broadcast channel
    let mut rx = tx.subscribe();

    // Spawn a task to send messages from the broadcast channel to the WebSocket client
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Err(_) = ws_tx.send(msg).await {
                break; // Client disconnected
            }
        }
    });

    // Handle incoming messages from the WebSocket client
    while let Some(result) = ws_rx.next().await {
        if let Ok(msg) = result {
            // Broadcast the received message to all connected clients
            if let Err(_) = tx.send(msg) {
                break; // Broadcast channel closed
            }
        } else {
            break; // WebSocket stream closed
        }
    }
}
