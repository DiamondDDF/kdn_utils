#![allow(warnings)]
#![allow(unused_imports)]
#![allow(warnings)]
#![allow(unused_imports)]
pub mod error;
mod input;
mod traits;
mod types;
pub use anyhow::*;
pub use console;
pub use error::*;
pub use input::*;
pub use thiserror::*;
pub use traits::*;
pub use types::*;

pub const NAMASTE: &str = &"🪬 ";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usd() {
        println!("Made change.");
    }
}
