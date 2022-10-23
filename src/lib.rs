pub mod traits;
pub mod types;
pub use traits::*;
pub use types::*;

pub const namaste: &str = "🪬";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usd() {
        println!("Made change.");
    }
}
