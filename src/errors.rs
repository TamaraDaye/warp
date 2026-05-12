use super::*;

#[derive(Debug, Error)]
pub struct WarpError<E: Error> {
    error: E,
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Invalid address could not parse")]
    ReceiverError(#[source] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("could not parse port number")]
    PortParseError(#[source] ParseIntError),

    #[error("Not a valid file")]
    InvalidFile(String),

    #[error("")]
    Arg(String),


    #[error("could not parse provided ip address")]
    InvalidIpAddress(#[source] AddrParseError)
}

#[derive(Debug, Error)]
pub enum DiskError {
    #[error("could not open file '{filename}'")]
    FileOpenError {
        filename: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Could not read file '{filename}'")]
    FileUnreadableError {
        filename: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid file name")]
    InvalidFile(String)
    
}
