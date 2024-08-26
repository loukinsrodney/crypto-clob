use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde_json::Value;
use std::sync::mpsc::Sender;
use crate::order_book::OrderBook; // Import the OrderBook struct

pub async fn subscribe_kraken_order_book(symbol: &str, tx: Sender<(String, OrderBook)>) {
    let kraken_symbol = format!("{}{}", symbol[..3].to_uppercase(), symbol[3..].to_uppercase());
    let url = "wss://ws.kraken.com";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut _write, mut read) = ws_stream.split();

    let subscription_message = serde_json::json!({
        "event": "subscribe",
        "pair": [kraken_symbol],
        "subscription": {
            "name": "book",
            "depth": 10
        }
    });

    _write.send(Message::Text(subscription_message.to_string()))
        .await
        .expect("Failed to send subscription message");

    let mut order_book = OrderBook::new();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let data: Value = serde_json::from_str(&text).expect("Failed to parse message");
                if let Some(arr) = data.as_array() {
                    if arr.len() > 1 {
                        if let Some(book_data) = arr.get(1) {
                            if let Some(bids) = book_data.get("b") {
                                for bid in bids.as_array().unwrap() {
                                    let price = bid[0].as_str().unwrap().parse::<f64>().unwrap();
                                    let quantity = bid[1].as_str().unwrap().parse::<f64>().unwrap();
                                    order_book.update("bids", price, quantity);
                                }
                            }
                            if let Some(asks) = book_data.get("a") {
                                for ask in asks.as_array().unwrap() {
                                    let price = ask[0].as_str().unwrap().parse::<f64>().unwrap();
                                    let quantity = ask[1].as_str().unwrap().parse::<f64>().unwrap();
                                    order_book.update("asks", price, quantity);
                                }
                            }

                            // Send the updated order book back through the channel
                            tx.send((symbol.to_string(), order_book.clone()))
                                .expect("Failed to send updated order book");
                        }
                    }
                }
            }
            _ => {}
        }
    }
}