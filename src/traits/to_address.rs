use crate::numbers::*;
use crate::traits::*;
use crate::error::*;
use anyhow::Result;
use anyhow::Error;
use anyhow::anyhow;
pub trait ToAddress {
    fn to_address(&self) -> Result<Address, anyhow::Error>;
}

impl<T> ToAddress for T where T: ToUnpadded + std::fmt::Debug {
    fn to_address(&self) -> anyhow::Result<Address> {
        let bytes = 
            self.bytes_unpadded();

        if bytes.len() != 20 {
            Err(anyhow!("address should have been 20 bytes. Instead it was {}.", bytes.len()))
        }
        else{
            Ok(Address::from_slice(bytes.into_iter().take(20).collect::<Vec<u8>>().as_ref()))
        }
    }
}
