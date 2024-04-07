//! Error types.

/// Any range of errors encountered when making requests.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An input parameter is missing or invalid.
    #[error("Invalid parameter: {}", .0)]
    Parameter(#[from] ParameterError),
    /// An error was encountered making a request.
    #[error("Request error: {}", .0)]
    Reqwest(#[from] reqwest::Error),
    #[error("Request middleware error: {}", .0)]
    /// An error was encountered within the request middleware.
    ReqwestMiddleware(anyhow::Error),
    #[error("Error parsing response: {}", .0)]
    /// An error was encountered parsing a JSON response body.
    Parse(#[from] serde_json::Error),
    #[error("{}", .0.status())]
    /// An error was encountered on response. This is usually a response with an HTTP code other 
    /// than 200. Check the status code of the response for more information.
    Http(reqwest::Response),
    /// Unexpected response. Check the message for more details.
    #[error("Unexpected response: {}", .0)]
    Response(String),
}

impl From<reqwest_middleware::Error> for Error {
    fn from(error: reqwest_middleware::Error) -> Self {
        match error {
            reqwest_middleware::Error::Reqwest(e) => {
                Self::Reqwest(e)
            },
            reqwest_middleware::Error::Middleware(e) => {
                Self::ReqwestMiddleware(e)
            },
        }
    }
}

/// Any number of issues with a provided parameter.
#[derive(thiserror::Error, Debug)]
pub enum ParameterError {
    /// Token is missing.
    #[error("Missing token")]
    MissingToken,
    /// Key is missing.
    #[error("Missing key")]
    MissingKey,
    /// An input parameter was empty.
    #[error("Provided {} is empty", .name)]
    Empty {
        /// The name of the parameter.
        name: &'static str,
    },
    /// An input parameter included too many values.
    #[error("Provided {} exceeded maximum length of {}", .name, .max)]
    MaximumLengthExceeded {
        /// The name of the parameter.
        name: &'static str,
        /// The maximum length of the parameter.
        max: usize,
    },
}

/// Error converting response currencies to currencies.
#[derive(Debug, thiserror::Error)]
pub enum TryFromResponseCurrenciesError {
    /// Currencies are cash currencies.
    #[error("Currencies are cash currencies")]
    IsCash,
    /// Error converting float currencies to currencies.
    #[error("Error converting float currencies to currencies: {}", .0)]
    TryFromFloatCurrenciesError(#[from] tf2_price::error::TryFromFloatCurrenciesError),
}