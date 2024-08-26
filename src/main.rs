mod exchanges;
mod consolidator;
mod order_book;  // Import the order_book module

use std::env;

use std::sync::mpsc::{self, Sender, Receiver};
use crate::order_book::OrderBook;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run -- <symbol> <exchange>");
        return;
    }
    let symbol = args[1].clone();
    let exchange = args[2].to_lowercase();

    let (tx, rx): (Sender<(String, OrderBook)>, Receiver<(String, OrderBook)>) = mpsc::channel();

    match exchange.as_str() {
        "kraken" => {
            let tx_kraken = tx.clone();
            let symbol_kraken = symbol.clone();
            tokio::spawn(async move {
                exchanges::kraken::subscribe_kraken_order_book(&symbol_kraken, tx_kraken).await;
            });
        }
        _ => {
            eprintln!("Unsupported exchange: {}", exchange);
            return;
        }
    }

    consolidator::consolidate_order_books(rx);
}
