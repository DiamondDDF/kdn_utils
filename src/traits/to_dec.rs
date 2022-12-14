#![allow(warnings)]
#![allow(unused_imports)]
use ethereum_types::{FromDecStrErr, U256};
use rust_decimal::prelude::*;
use rust_decimal_macros::*;
use thiserror::Error;

pub trait ToDec {
    fn to_dec(&self) -> Decimal;
}
impl ToDec for U256 {
    fn to_dec(&self) -> Decimal {
        Decimal::from_str_radix(&self.to_string(), 10).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dec_to_u256() {
        let num = U256::from_dec_str("500000000000").unwrap();
        let dec = num.to_dec();
    }
}
