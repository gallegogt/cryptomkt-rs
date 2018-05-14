use std::collections::HashMap;
use internal::api;
use internal::errors::CryptoMktErrorType;
use serde::de::DeserializeOwned;

///
/// Métodos de encuestas soportados por el API
///
pub enum RequestMethod {
    /// HTTP Request POST
    Post,
    /// HTTP Request GET, El parámetro define si la petición es para un
    /// endpoint publico o no
    Get(bool),
}

///
/// Cryptomarket API
///
pub struct CryptoMktApi<'a> {
    i_api: Box<api::Api<'a>>,
}

impl<'a> CryptoMktApi<'a> {
    ///
    /// Crea la nueva instancia del API
    ///
    pub fn new(api_key: &'a str, secret_key: &'a str) -> Self {
        CryptoMktApi {
            i_api: Box::new(api::Api::new(api_key, secret_key, None)),
        }
    }

    ///
    /// Devuelve el dominio
    ///
    pub fn domain(&self) -> &'a str {
        self.i_api.domain()
    }

    ///
    /// Devuelve la version del API
    ///
    pub fn version(&self) -> &'a str {
        self.i_api.api_version()
    }

    ///
    /// Peticion HTTP
    ///
    /// #Argumentos
    ///     method: Get(is_public) | Post
    ///     endpoint: Cadena de texto con el endpoint de la API (Ej: "orders/active" )
    ///     payload: Datos a enviar endpoint
    ///
    pub fn call<T>(
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
