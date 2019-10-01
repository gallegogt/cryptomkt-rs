//!
//! En este módulo se encuentra las diferentes respuestas dada por el
//! servidor para los diferentes endpoints
//!

use crate::internal::models::{Balance, Book, Order, OrdersInstant, Payment, Ticker, Trade};
use serde::Deserialize;
use serde_json::Value;

/// Información sobre la paginación
#[derive(Deserialize, Debug, Clone)]
pub struct Pagination {
    pub limit: i32,
    #[serde(default)]
    pub page: i32,
    #[serde(default)]
    pub previous: Value, // String / i32
    pub next: Value, // String / i32
}
/// Timeout in seconds.
impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            limit: 0,
            page: 0,
            previous: Value::default(),
            next: Value::default(),
        }
    }
}

/// Conforma la respuesta enviada por el Exchange cryptomkt
#[derive(Deserialize, Debug, Clone)]
pub struct CryptoMktResponse<T> {
    pub status: String,
    pub data: T,
    #[serde(default)]
    pub pagination: Pagination,
}

// ============ Market ==============

/// Estructura de la respuesta asociada a los mercados, existentes
pub type MarketResponse = CryptoMktResponse<Vec<String>>;

/// Respuesta asociada al Ticker
pub type TickerResponse = CryptoMktResponse<Vec<Ticker>>;

/// Lista de órdenes activas en CryptoMarket.
pub type BookResponse = CryptoMktResponse<Vec<Book>>;

/// Retorna listado de trades realizados en CryptoMarket.
pub type TradeResponse = CryptoMktResponse<Vec<Trade>>;

/// Listados de Ordennes
pub type OrderResponse = CryptoMktResponse<Vec<Order>>;

/// Respuesta Asignada a la accion Crear Orden
pub type SimpleOrderResponse = CryptoMktResponse<Order>;

/// Permite obtener en base al estado actual del mercado, la cantidad de criptomonedas o
/// moneda local a recibir si se ejecuta una compra o venta respectivamente.
pub type OrdersInstantResponse = CryptoMktResponse<OrdersInstant>;
pub type EmptyResponse = CryptoMktResponse<String>;

/// Obtener balances:
pub type BalanceResponse = CryptoMktResponse<Vec<Balance>>;

/// Pagos:
pub type PaymentResponse = CryptoMktResponse<Payment>;
pub type PaymentListResponse = CryptoMktResponse<Vec<Payment>>;
