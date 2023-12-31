use std::env;
use std::io::{self};

fn main() {
    let args: Vec<String> = env::args().collect();

    let character = if args.len() > 1 {
        args[1].chars().next().expect("No character found")
    } else {
        //io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().chars().next().expect("No character found")
    };

    let mut utf8_bytes_hex = String::new();

    for byte in character.to_string().as_bytes() {
        utf8_bytes_hex.push_str(&format!("{:02X} ", byte));
    }

    utf8_bytes_hex = utf8_bytes_hex.trim_end().to_string();

    let unicode_value = format!("U+{:04X}", character as u32);
    println!("{}|{}", unicode_value, utf8_bytes_hex);
}

