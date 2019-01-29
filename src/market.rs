use api::{CryptoMktApi, RequestMethod};
use internal::errors::CryptoMktResult;
use internal::models::{Book, Order, OrdersInstant, Ticker, Trade};
use internal::response::{BookResponse, EmptyResponse, OrderResponse, OrdersInstantResponse,
                         SimpleOrderResponse, TickerResponse, TradeResponse};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

///
/// Order Type
///
#[derive(Debug)]
pub enum OrderType {
    Buy,
    Sell,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

///
/// Order State
///
pub enum OrderState {
    Active,
    Executed,
}

///
/// Market
///
/// Through this class you access the functionalities offered by the market,
/// either create purchase order, choose the state of the market, etc ...
///
pub struct Market {
    api: CryptoMktApi,
    name: String,
}

impl Market {
    ///
    /// Create new instance
    ///
    pub fn new<'m>(api: CryptoMktApi, market_name: &'m str) -> Self {
        Market {
            api: api,
            name: market_name.to_string(),
        }
    }

    ///
    /// Get the market name (Ej ETHCLP)
    ///
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    ///
    /// Get the current ticker
    ///
    pub fn get_current_ticker(&self) -> CryptoMktResult<Ticker> {
        let mut params = HashMap::new();
        params.insert("market".to_string(), self.name.clone());
        let resp = self.api
            .call::<TickerResponse>(RequestMethod::Get(true), "ticker", params);
        match resp {
            Ok(value) => Ok(value.data[0].clone()),
            Err(e) => Err(e),
        }
    }

    ///
    /// Get the order books
    ///
    pub fn get_orders_book(
        &self,
        orders_type: OrderType,
        page: u32,
        limit: u32,
    ) -> CryptoMktResult<Vec<Book>> {
        let mut params = HashMap::new();
        params.insert("market".to_string(), self.name.clone());
        params.insert("type".to_string(), orders_type.to_string().to_lowercase());
        params.insert("page".to_string(), format!("{}", page));
        params.insert("limit".to_string(), format!("{}", limit));

        let resp = self.api
            .call::<BookResponse>(RequestMethod::Get(true), "book", params);
        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }

    ///
    /// Get Trades
    ///
    pub fn get_trades<'m>(
        &self,
        start: &'m str,
        end: &'m str,
        page: u32,
        limit: u32,
    ) -> CryptoMktResult<Vec<Trade>> {
        let mut params = HashMap::new();
        params.insert("market".to_string(), self.name.clone());
        params.insert("start".to_string(), start.to_string());
        params.insert("end".to_string(), end.to_string());
        params.insert("page".to_string(), format!("{}", page));
        params.insert("limit".to_string(), format!("{}", limit));

        let resp = self.api
            .call::<TradeResponse>(RequestMethod::Get(true), "trades", params);
        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }

    ///
    /// Get user orders by state
    ///
    pub fn get_user_orders_by_state(
        &self,
        state: OrderState,
        page: u32,
        limit: u32,
    ) -> CryptoMktResult<Vec<Order>> {
        let mut params = HashMap::new();
        params.insert("market".to_string(), self.name.clone());
        params.insert("page".to_string(), format!("{}", page));
        params.insert("limit".to_string(), format!("{}", limit));

        let endpoint = match state {
            OrderState::Active => "orders/active",
            OrderState::Executed => "orders/executed",
        };

        let resp = self.api
            .call::<OrderResponse>(RequestMethod::Get(false), endpoint, params);
        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }
    ///
    /// Create order
    ///
    pub fn create_order(
        &self,
        order_type: OrderType,
        amount: f32,
        price: f32,
    ) -> CryptoMktResult<Vec<Order>> {
        let mut params = HashMap::new();
        params.insert("market".to_string(), self.name.clone());
        params.insert("amount".to_string(), format!("{}", amount));
        params.insert("price".to_string(), format!("{}", price));
        params.insert("type".to_string(), order_type.to_string().to_lowercase());

        let resp = self.api
            .call::<OrderResponse>(RequestMethod::Post, "orders/create", params);
        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }

    ///
    /// Get Order status
    ///
    pub fn get_order_status<'m>(&self, order_id: &'m str) -> CryptoMktResult<Order> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), order_id.to_string());

        let resp = self.api.call::<SimpleOrderResponse>(
            RequestMethod::Get(false),
            "orders/status",
            params,
        );
        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }

    ///
    /// Cancel Order
    ///
    pub fn cancel_order<'m>(&self, order_id: &'m str) -> CryptoMktResult<Order> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), order_id.to_string());

        let resp =
            self.api
                .call::<SimpleOrderResponse>(RequestMethod::Post, "orders/cancel", params);
        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }

    ///
    /// Get order instant
    ///
    /// An instant order corresponds to a purchase or sale request within the
    /// Instant Exchange of CryptoMarket.
    ///
    pub fn get_order_instant(
        &self,
        order_type: OrderType,
        amount: f32,
    ) -> CryptoMktResult<OrdersInstant> {
        let mut params = HashMap::new();
        params.insert("market".to_string(), self.name.clone());
        params.insert("amount".to_string(), format!("{}", amount));
        params.insert("type".to_string(), order_type.to_string().to_lowercase());

        let resp = self.api.call::<OrdersInstantResponse>(
            RequestMethod::Get(false),
            "orders/instant/get",
            params,
        );
        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }

    ///
    /// Create an instant order in the Instant Exchange of CryptoMarket
    ///
    pub fn create_order_instant(
        &self,
        order_type: OrderType,
        amount: f32,
    ) -> CryptoMktResult<String> {
        let mut params = HashMap::new();
        params.insert("market".to_string(), self.name.clone());
        params.insert("amount".to_string(), format!("{}", amount));
        params.insert("type".to_string(), order_type.to_string().to_lowercase());

        let resp =
            self.api
                .call::<EmptyResponse>(RequestMethod::Post, "orders/instant/create", params);
        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }
}
