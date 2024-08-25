use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use crate::order_book::OrderBook; // Import the OrderBook struct

pub fn consolidate_order_books(rx: Receiver<(String, OrderBook)>) {
    let order_books: Arc<Mutex<HashMap<String, OrderBook>>> = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((exchange, order_book)) = rx.recv() {
        let mut books = order_books.lock().unwrap();
        books.insert(exchange.clone(), order_book.clone());

        // Print consolidated top of book across all exchanges
        println!("Consolidated Top of Book:");
        for (exch, book) in books.iter() {
            let (top_bid, top_ask) = book.top_of_book();
            println!(
                "{}: Bid: {:?}, Ask: {:?}",
                exch,
                top_bid.unwrap_or((0.0, 0.0)),
                top_ask.unwrap_or((0.0, 0.0))
            );
        }
    }
}

