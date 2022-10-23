use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub trait ToUSD {
    fn usd(&self)-> Decimal;
}