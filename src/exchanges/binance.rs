use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use serde_json::Value;
use std::sync::mpsc::Sender;

pub async fn subscribe_binance_order_book(symbol: &str, tx: Sender<Value>) {
    // Construct the WebSocket URL dynamically based on the symbol
    let url = format!("wss://stream.binance.com:9443/ws/{}@depth", symbol);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut _write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let data: Value = serde_json::from_str(&text).expect("Failed to parse message");
                tx.send(data).expect("Failed to send data");
            }
            _ => {}
        }
    }
}
