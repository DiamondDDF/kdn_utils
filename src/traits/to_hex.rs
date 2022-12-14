#![allow(warnings)]
#![allow(unused_imports)]
pub trait ToHex {
    fn hex(&self) -> String;
}
impl ToHex for u128 {
    fn hex(&self) -> String {
        format!("{:x}", self)
    }
}

impl ToHex for &str {
    fn hex(&self) -> String {
        let hex_str = self.trim_start_matches("0x").to_string();
        hex::decode(&hex_str).unwrap();

        hex_str
    }
}
impl ToHex for Vec<u8> {
    fn hex(&self) -> String {
        hex::encode(self)
    }
}
impl ToHex for [u8] {
    fn hex(&self) -> String {
        hex::encode(self)
    }
}
