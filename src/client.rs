use api::{CryptoMktApi, RequestMethod};
use market::Market;

use std::collections::HashMap;
use internal::response::{MarketResponse, BalanceResponse, Balance};
use internal::errors::CryptoMktResult;

///
/// Cliente Cryptomkt
///
pub struct CryptoMktClient {
    api: CryptoMktApi
}

impl CryptoMktClient {
    ///
    /// Inicializador
    ///
    pub fn new<'a>(api_key: &'a str, secret_key: &'a str) -> Self {
        CryptoMktClient {
            api: CryptoMktApi::new(api_key, secret_key)
        }
    }
    ///
    /// Devuelve el listado de Mercados existentes en CryptoMkt
    ///
    pub fn get_markets(&self) -> Vec<Market> {
        let resp = self.api.call::<MarketResponse>(RequestMethod::Get(true), "market", HashMap::new());
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
    /// Un balance corresponde al estado de tus billeteras de criptomonedas y locales. Este estado contiene el saldo disponible, saldo contable y billetera correspondiente.
    ///
    pub fn get_balance(&self) -> CryptoMktResult<Vec<Balance>> {
        let resp = self.api.call::<BalanceResponse>(RequestMethod::Get(false), "balance", HashMap::new());
        match resp {
            Ok(value) => {
                Ok(value.data)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Get API
    pub fn get_api(&self) -> &CryptoMktApi {
        &self.api
    }
}
