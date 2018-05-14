extern crate reqwest;

use self::reqwest::Client;
use self::reqwest::header::Headers;
use self::reqwest::{StatusCode, Url};
use std::collections::HashMap;

use internal::errors::{CryptoMktErrorType,CryptoMktResult};

pub trait HttpReq {
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: Headers
    ///
    fn get(&self, url: Url, headers: Headers) -> Result<String, CryptoMktErrorType>;
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: Headers
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn post(
        &self,
        url: Url,
        headers: Headers,
        payload: HashMap<String, String>,
    ) -> Result<String, CryptoMktErrorType>;
}

///
/// CryptoMktRequest
///
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
            StatusCode::Unauthorized => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::Unauthorized);
                CryptoMktErrorType::RequestUnauthorized
            }

            StatusCode::Forbidden => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::Forbidden);
                CryptoMktErrorType::RequestForbidden
            }
            StatusCode::NotFound => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::NotFound);
                CryptoMktErrorType::RequestNotFound
            }
            StatusCode::MethodNotAllowed => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::MethodNotAllowed);
                CryptoMktErrorType::RequestMethodNotAllowed
            }
            StatusCode::NotAcceptable => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::NotAcceptable);
                CryptoMktErrorType::RequestNotAcceptable
            }
            StatusCode::Gone => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::Gone);
                CryptoMktErrorType::RequestGone
            }
            StatusCode::TooManyRequests => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::TooManyRequests);
                CryptoMktErrorType::RequestTooManyRequests
            }
            StatusCode::InternalServerError => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::InternalServerError);
                CryptoMktErrorType::RequestInternalServerError
            }
            StatusCode::ServiceUnavailable => {
                error!(target: "cryptomkt", "{}: StatusCode: {:?}", prefix, StatusCode::ServiceUnavailable);
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
    ///     headers: Headers
    ///
    fn get(&self, url: Url, headers: Headers) -> CryptoMktResult<String> {
        let result = self.client.get(url).headers(headers).send();
        match result {
            Ok(mut resp) => match resp.status() {
                StatusCode::Ok => match resp.text() {
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
    ///     headers: Headers
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn post(
        &self,
        url: Url,
        headers: Headers,
        payload: HashMap<String, String>,
    ) -> CryptoMktResult<String> {
        let result = self.client.post(url).headers(headers).form(&payload).send();

        match result {
            Ok(mut resp) => match resp.status() {
                StatusCode::Ok => match resp.text() {
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
