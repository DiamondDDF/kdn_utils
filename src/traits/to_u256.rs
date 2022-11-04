
use rust_decimal::prelude::*;
use rust_decimal_macros::*;
use thiserror::Error;
use ethereum_types::{
    U256,
    FromDecStrErr
};
#[derive(Error, Debug)]
pub enum ConvError {
    #[error("Tried to convert to an integer from a string that has fractions. Value was {value}")]
    Truncation{ 
        value: String
    },
    #[error("Invalid character. Probably not a number. Source: {0}")]
    InvalidCharacter(String),
    #[error("Invalid Length.")]
    InvalidLength(String)
}
impl From<FromDecStrErr> for ConvError{
    fn from(err: FromDecStrErr) -> Self {
        match err {
            FromDecStrErr::InvalidCharacter => ConvError::InvalidCharacter(err.to_string()),
            FromDecStrErr::InvalidLength => ConvError::InvalidLength(err.to_string())
        }
    }
}
pub trait ToU256 {
    fn to_u256(&self) -> U256;
    fn to_u256_checked(&self) -> Result<U256, ConvError>;
}

impl ToU256 for Decimal {
    fn to_u256(&self) -> U256 {
        U256::from_dec_str(&self.round().to_string()).unwrap()     
    }

    fn to_u256_checked(&self) -> Result<U256, ConvError> {
        if self.fract() > dec!(0) {
            return Err(ConvError::Truncation { value: self.to_string() })
        }
        else {
            Ok(U256::from_dec_str(&self.to_string())?)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fractional_conversion(){
        let dec = dec!(2.5);
        let num = dec.to_u256();
        println!("{num}");
    }
}