# [UNOFFICIAL] Criptomarket API

Library for access to the cryptomarket platform (https://www.cryptomkt.com)

## Example

```rust
///
/// Example of access to the Public API of cryptomkt
///
extern crate cryptomkt;

use cryptomkt::{CryptoMktClient, OrderType};

const API_KEY: &'static str = "<API_KEY>";
const API_SECRET: &'static str = "<API SECRET>";

fn main() {
    let client = CryptoMktClient::new(API_KEY, API_SECRET);

    // Get Markets
    let markets = client.get_markets();
    for m in markets.iter() {
        println!("{}", m.get_name());

        // Get Current Ticker
        match m.get_current_ticker() {
            Ok(ticker) => {
                println!("{:?}", ticker);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

        // Get Orders Book
        println!("------- Orders ------");
        match m.get_orders_book(OrderType::Buy, 0, 20) {
            Ok(orders) => {
                println!("{:?}", orders);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

        // Get Trades
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

```


# Contributing

You want to contribute to this project? Wow, thanks! So please just fork it and send me a pull request.

# Support My Efforts

I programmed this lib for fun and do my best effort to support those that have issues with it, please return the favor and support me.

[![paypal](https://www.paypalobjects.com/en_US/i/btn/btn_donateCC_LG.gif)](https://www.paypal.com/donate?hosted_button_id=T2E5TYZWRECMW)
