use std::io::{
    self,
    Write,
    Read
};

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
pub fn prompt(message: &str) -> String {
    let mut in_str = String::new();
    println!("{}", message);
    let _ = io::stdin().read_line(&mut in_str);
    let in_str = in_str
        .to_uppercase()
        .replace("\n", "")
        .trim().to_string();

    in_str
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pause(){
        pause();
    }
}