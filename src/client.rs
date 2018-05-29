use api::{CryptoMktApi, RequestMethod};
use market::Market;

use internal::errors::CryptoMktResult;
use internal::models::{Balance, Payment};
use internal::response::{BalanceResponse, MarketResponse, PaymentResponse, PaymentListResponse};
use std::collections::HashMap;

///
/// Implementación del cliente para el API
///
/// # Ejemplo
///
/// En este ejemplo se imrpimen el tricker actual de todos los mercados disponibles
///
/// ```
/// extern crate cryptomkt;
/// use cryptomkt::{CryptoMktClient, OrderType};
///
/// const API_KEY: &'static str = "<API_KEY>";
/// const API_SECRET: &'static str = "<API SECRET>";
///
/// let client = CryptoMktClient::new(API_KEY, API_SECRET);
/// let markets = client.get_markets();
/// for m in markets.iter() {
///     println!("{}", m.get_name());
///     match m.get_current_ticker() {
///         Ok(ticker) => {
///           println!("{:?}", ticker);
///         }
///         Err(e) => {
///            println!("{:?}", e);
///         }
///       }
///     }
/// ```
///
pub struct CryptoMktClient {
    api: CryptoMktApi,
}

impl CryptoMktClient {
    ///
    /// Inicializador
    ///
    pub fn new<'a>(api_key: &'a str, secret_key: &'a str) -> Self {
        CryptoMktClient {
            api: CryptoMktApi::new(api_key, secret_key),
        }
    }
    ///
    /// Devuelve el listado de Mercados existentes en CryptoMkt
    ///
    pub fn get_markets(&self) -> Vec<Market> {
        let resp =
            self.api
                .call::<MarketResponse>(RequestMethod::Get(true), "market", HashMap::new());
        match resp {
            Ok(value) => {
                let mut market_list = Vec::new();
                for it in value.data {
                    market_list.push(Market::new(self.api.clone(), it.clone().as_str()));
                }
                market_list
            }
            Err(e) => {
                println!("{:?}", e);
                Vec::new()
            }
        }
    }
    ///
    /// Devuelve un objeto Mercado dado el nombre
    ///
    pub fn create_market<'a>(&self, name: &'a str) -> Market {
        Market::new(self.api.clone(), name)
    }
    ///
    /// Un balance corresponde al estado de tus billeteras de criptomonedas y locales.
    /// Este estado contiene el saldo disponible, saldo contable y billetera correspondiente.
    ///
    pub fn get_balance(&self) -> CryptoMktResult<Vec<Balance>> {
        let resp =
            self.api
                .call::<BalanceResponse>(RequestMethod::Get(false), "balance", HashMap::new());
        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }

    ///
    /// Permite crear una orden de pago, entregando QR y urls para pagar.
    ///
    pub fn create_payment_order<'a>(
        &self,
        to_receive: f32,
        to_receive_currency: &'a str,
        payment_receiver: &'a str,
        external_id: Option<String>,
        callback_url: Option<String>,
        error_url: Option<String>,
        success_url: Option<String>,
        refund_email: Option<String>,
    ) -> CryptoMktResult<Payment> {
        let mut params = HashMap::new();
        params.insert("to_receive".to_string(), format!("{}", to_receive));
        params.insert(
            "to_receive_currency".to_string(),
            to_receive_currency.to_string(),
        );
        params.insert("payment_receiver".to_string(), payment_receiver.to_string());

        if let Some(external_id) = external_id {
            params.insert("external_id".to_string(), external_id);
        }
        if let Some(callback_url) = callback_url {
            params.insert("callback_url".to_string(), callback_url);
        }
        if let Some(error_url) = error_url {
            params.insert("error_url".to_string(), error_url);
        }
        if let Some(success_url) = success_url {
            params.insert("success_url".to_string(), success_url);
        }
        if let Some(refund_email) = refund_email {
            params.insert("refund_email".to_string(), refund_email);
        }

        let resp =
            self.api
                .call::<PaymentResponse>(RequestMethod::Post, "payment/new_order", params);
        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }

    ///
    /// Permite crear una orden de pago, entregando QR y urls para pagar.
    ///
    pub fn payment_order_status<'a>(&self, id: &'a str) -> CryptoMktResult<Payment> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), id.to_string());

        let resp =
            self.api
                .call::<PaymentResponse>(RequestMethod::Get(false), "payment/status", params);

        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }

    ///
    /// Retorna el listado de las órdenes de pago generadas
    ///
    pub fn get_payment_orders<'a>(
        &self,
        start_date: &'a str,
        end_date: &'a str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> CryptoMktResult<Vec<Payment>> {
        let mut params = HashMap::new();
        params.insert("start_date".to_string(), start_date.to_string());
        params.insert("end_date".to_string(), end_date.to_string());

        if let Some(page) = page {
            params.insert("page".to_string(), format!("{}", page));
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), format!("{}", limit));
        }

        let resp =
            self.api
                .call::<PaymentListResponse>(RequestMethod::Get(false), "payment/status", params);

        match resp {
            Ok(value) => Ok(value.data),
            Err(e) => Err(e),
        }
    }
}
