pub use thiserror::Error;
pub use thiserror::*;
pub use anyhow;
pub use anyhow::*;
pub use enum_map::*;
pub use enum_map_derive::Enum;
use std::io;

#[cfg(test)]
mod examples {
    use std::backtrace::Backtrace;

    use super::*;
    #[derive(Error, Debug)]
    pub enum DataStoreError {
        #[error("data store disconnected")]
        Disconnect(#[from] io::Error),
        #[error("the data for key `{0}` is not available")]
        Redaction(String),
        #[error("invalid header (expected {expected:?}, found {found:?})")]
        InvalidHeader {
            expected: String,
            found: String,
        },
        #[error("unknown data store error")]
        Unknown,
    }
    
    #[derive(Error, Debug)]
    pub enum MyError {
        #[error("Io error:\nSource:{source}\n")]
        Io {
            
            source: io::Error,
            //backtrace: Backtrace,
        },
    }
}