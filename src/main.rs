mod encoder;
mod decoder;
mod tree;
mod util;

use std::process::exit;
use std::error::Error;


fn main() -> Result <(), Box<dyn Error>> {

    let commands: Vec<_> = std::env::args().collect();

    if commands.len() < 3 {
        eprintln!("Usage: huffit <argument> <input_file>");
        exit(1);
    }

    let input_file = &commands[2];

    match commands[1].as_str() {

        "--compress" => {
            if let Err(e) = encoder::encode(input_file.to_string(), "compressed.txt".to_string()) {
                eprintln!("Encoding error: {e}");
                exit(1);
            }
        },

        "--decompress" => {
            if let Err(e) = decoder::decode(input_file.to_string(), "decompressed.txt".to_string()) {
                eprintln!("Decoding error: {e}");
                exit(1);
            }
        },
        _ => {
            eprintln!("Unknown command: {}", commands[1]);
            exit(1);
        }
    }

    Ok(())
}
