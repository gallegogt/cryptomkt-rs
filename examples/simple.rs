///
/// Example
///
extern crate cryptomkt;

use cryptomkt::{CryptoMktClient, OrderType};

const API_KEY: &'static str = "<API_KEY>";
const API_SECRET: &'static str = "<API SECRET>";

fn main() {
    let client = CryptoMktClient::new(API_KEY, API_SECRET);

    // Get all markets available
    let markets = client.get_markets();
    for m in markets.iter() {
        println!("{}", m.get_name());

        // Get the current ticker for the market
        match m.get_current_ticker() {
            Ok(ticker) => {
                println!("{:?}", ticker);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

        println!("------- Orders ------");
        match m.get_orders_book(OrderType::Buy, 0, 20) {
            Ok(orders) => {
                println!("{:?}", orders);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

        println!("------- Trades ------");
        match m.get_trades("2018-05-15", "2018-05-16", 0, 20) {
            Ok(trades) => {
                println!("{:?}", trades);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
