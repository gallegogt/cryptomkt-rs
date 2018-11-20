extern crate reqwest;

use self::reqwest::Client;
use reqwest::header::HeaderMap;
use self::reqwest::{StatusCode, Url};
use std::collections::HashMap;

use internal::errors::{CryptoMktErrorType,CryptoMktResult};

///
/// Definición que deben cumplir para poder extaer datos mediante HTTP
///
pub trait HttpReq {
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: HeaderMap
    ///
    fn get(&self, url: Url, headers: HeaderMap) -> Result<String, CryptoMktErrorType>;
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: Headers
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn post(
        &self,
        url: Url,
        headers: HeaderMap,
        payload: HashMap<String, String>,
    ) -> Result<String, CryptoMktErrorType>;
}

///
/// CryptoMktRequest
///
#[derive(Debug, Clone)]
pub struct CryptoMktRequest {
    client: Box<Client>,
}

impl CryptoMktRequest {
    ///
    /// Devuelve una nueva instancia
    ///
    pub fn new() -> Self {
        CryptoMktRequest {
            client: Box::new(Client::new()),
        }
    }
    ///
    /// Traspasa los errores del StatusCode para CryptoMktErrorType
    ///
    /// Argumentos:
    ///     prefix: Cadena de texto adiciona al log de errores
    ///     status: Estado de la petición
    ///
    pub fn translate_errors<'c>(&self, prefix: &'c str, status: StatusCode) -> CryptoMktErrorType {
        match status {
            StatusCode::UNAUTHORIZED => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::UNAUTHORIZED);
                CryptoMktErrorType::RequestUnauthorized
            }

            StatusCode::FORBIDDEN => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::FORBIDDEN);
                CryptoMktErrorType::RequestForbidden
            }
            StatusCode::NOT_FOUND => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::NOT_FOUND);
                CryptoMktErrorType::RequestNotFound
            }
            StatusCode::METHOD_NOT_ALLOWED => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::METHOD_NOT_ALLOWED);
                CryptoMktErrorType::RequestMethodNotAllowed
            }
            StatusCode::NOT_ACCEPTABLE => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::NOT_ACCEPTABLE);
                CryptoMktErrorType::RequestNotAcceptable
            }
            StatusCode::GONE => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::GONE);
                CryptoMktErrorType::RequestGone
            }
            StatusCode::TOO_MANY_REQUESTS => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::TOO_MANY_REQUESTS);
                CryptoMktErrorType::RequestTooManyRequests
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::INTERNAL_SERVER_ERROR);
                CryptoMktErrorType::RequestInternalServerError
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::SERVICE_UNAVAILABLE);
                CryptoMktErrorType::RequestServiceUnavailable
            }
            status => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?} Code({:?})", prefix, status, status.as_u16());
                if status.as_u16() == 418 {
                    CryptoMktErrorType::RequestTeapot
                } else {
                    CryptoMktErrorType::BadRequest
                }
            }
        }
    }
}

impl HttpReq for CryptoMktRequest {
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: HeaderMap
    ///
    fn get(&self, url: Url, headers: HeaderMap) -> CryptoMktResult<String> {
        let result = self.client.get(url).headers(headers).send();
        match result {
            Ok(mut resp) => match resp.status() {
                StatusCode::OK => match resp.text() {
                    Ok(txt) => Ok(txt),
                    Err(e) => {
                        error!(target: "cryptomkt", "GET: Request Text details: {:?}", e);
                        Err(CryptoMktErrorType::MalformedResource)
                    }
                },
                status => Err(self.translate_errors("GET", status)),
            },
            Err(e) => {
                error!(target: "cryptomkt", "GET {:?}", e);
                Err(CryptoMktErrorType::BadRequest)
            }
        }
    }
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: HeaderMap
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn post(
        &self,
        url: Url,
        headers: HeaderMap,
        payload: HashMap<String, String>,
    ) -> CryptoMktResult<String> {
        let result = self.client.post(url).headers(headers).form(&payload).send();

        match result {
            Ok(mut resp) => match resp.status() {
                StatusCode::OK => match resp.text() {
                    Ok(txt) => Ok(txt),
                    Err(e) => {
                        error!(target: "cryptomkt", "POST: Response Details: {:?}", e);
                        Err(CryptoMktErrorType::BadRequest)
                    }
                },
                status => Err(self.translate_errors("POST", status)),
            },
            Err(e) => {
                error!(target: "cryptomkt", "POST {:?}", e);
                Err(CryptoMktErrorType::BadRequest)
            }
        }
    }
}
