use crate::internal::api::Api;
use crate::internal::errors::CryptoMktErrorType;
use crate::internal::request::CryptoMktRequest;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

///
/// HTTP methods supported by the API
///
pub enum RequestMethod {
    /// HTTP Request POST
    Post,
    /// HTTP Request GET, The parameter defines whether the request is
    /// for a public endpoint or not
    Get(bool),
}

///
/// # Cryptomkt API
///
/// Allows access to the cryptomarket API. For more information about
/// endpoints go to [Developers Cryptomkt] (http://developers.cryptomkt.com/)
///
/// ## Example
///
/// ```
/// extern crate cryptomkt;
/// use cryptomkt::CryptoMktApi;
///
/// let api = CryptoMktApi::new("<API Key>", "<Secret Key>");
/// println!("API version: {}", api.version());
/// println!("API domain: {}", api.domain());
/// ```
///
#[derive(Debug, Clone)]
pub struct CryptoMktApi {
    i_api: Box<Api<CryptoMktRequest>>,
}

impl CryptoMktApi {
    ///
    /// Create the new API instance
    ///
    /// Arguments:
    ///     api_key: API Key as string
    ///     secret_key: Secret Key as string
    ///
    pub fn new<'a>(api_key: &'a str, secret_key: &'a str) -> Self {
        CryptoMktApi {
            i_api: Box::new(Api::<CryptoMktRequest>::new(
                api_key,
                secret_key,
                Box::new(CryptoMktRequest::new()),
            )),
        }
    }

    ///
    /// Get the domain
    ///
    pub fn domain(&self) -> String {
        self.i_api.domain()
    }

    ///
    /// Get the API Version
    ///
    pub fn version(&self) -> String {
        self.i_api.api_version()
    }

    ///
    /// Function that you can use to make request over Cryptomarket API
    ///
    /// ```
    /// extern crate cryptomkt;
    /// use cryptomkt::{CryptoMktApi, Market, RequestMethod};
    /// use cryptomkt::response::MarketResponse;
    /// use std::collections::HashMap;
    ///
    /// let api = CryptoMktApi::new("<API Key>", "<Secret Key>");
    /// let resp = api.call::<MarketResponse>(RequestMethod::Get(true), "market", HashMap::new());
    /// match resp {
    ///     Ok(value) => {
    ///         let mut market_list = Vec::new();
    ///         for it in value.data {
    ///             market_list.push(Market::new(api.clone(), it.clone().as_str()));
    ///         }
    ///         println!("{:?}", market_list[0].get_name());
    ///     }
    ///     Err(e) => {
    ///         println!("{:?}", e);
    ///     }
    /// }
    /// ```
    ///
    /// #Arguments
    ///     `method`: Enum representing the server request method: Get (is_public) | Post
    ///     `endpoint`: Endpoint
    ///     `payload`: Payload
    ///
    pub fn call<'a, T>(
        &self,
        method: RequestMethod,
        endpoint: &'a str,
        payload: HashMap<String, String>,
    ) -> Result<T, CryptoMktErrorType>
    where
        T: DeserializeOwned,
    {
        match method {
            RequestMethod::Get(is_public) => self.i_api.get_edge(endpoint, payload, is_public),
            RequestMethod::Post => self.i_api.post_edge(endpoint, payload),
        }
    }
}
