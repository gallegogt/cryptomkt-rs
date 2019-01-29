#[derive(Debug)]
pub enum CryptoMktErrorType {
    // 401 => Tu API key es errónea
    RequestUnauthorized,
    // 403 => El recurso solicitado solo está disponible para administradores.
    RequestForbidden,
    // 404 => El recurso solicitado no pudo ser encontrado.
    RequestNotFound,
    // 404 => El recurso solicitado no pudo ser encontrado.
    RequestMethodNotAllowed,
    // 406 => Solicitaste un formato que no es JSON.
    RequestNotAcceptable,
    // 410  => El recurso solicitado ha sido removido de nuestros servidores.
    RequestGone,
    // 418 => I'm a teapot.
    RequestTeapot,
    // 429 => Estás solicitando muchos recursos! Detente!
    RequestTooManyRequests,
    // 500 => Tenemos problemas en nuestros servidores. Inténtalo más tarde.
    RequestInternalServerError,
    // 503 => Estamos temporalmente fuera de línea por mantención. Por favor inténtalo más tarde.
    RequestServiceUnavailable,
    // 400 => Petición inválida
    BadRequest,
    //
    MalformedResource,
}

// Define alea generico al Result para  CryptoMktErrorType
pub type CryptoMktResult<T> = Result<T, CryptoMktErrorType>;
