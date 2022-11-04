mod traits;
mod input;
mod types;
pub mod error;
pub use traits::*;
pub use types::*;
pub use input::*;
pub use anyhow::*;
pub use thiserror::*;
pub use console;
pub use error::*;


pub const NAMASTE: &str = &"🪬 ";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usd() {
        println!("Made change.");
    }
}
