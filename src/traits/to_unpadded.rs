use hex;
use super::*;
pub trait ToUnpadded {
    fn bytes_unpadded(&self) -> Vec<u8>;
 }
 impl<T> ToUnpadded for T where T: ToHex {
    fn bytes_unpadded(&self) -> Vec<u8> {
        //odd length means, last digit will be 4 instead of 8 bits.
        let mut hex_str = self.hex();
        //TODO: find out if I need to somehow afterword remove the extra four bits.
        if hex_str.len() % 2 == 1 {
            hex_str = ["0".into(), hex_str].concat();
        }
        let bytes_unpadded  = hex::decode(hex_str).expect("This doesn't look like a hex number, can't parse it.");

        bytes_unpadded
    }
}
