use std::fs;
use std::collections::HashMap;
use std::error::Error;


pub fn decode(input_file: String, output_file: String) -> Result<(), Box<dyn Error>> {

    let file_bytes = fs::read(input_file)?;

    let marker = b"----\n";

    let marker_index = file_bytes
        .windows(marker.len())
        .position(|window| window == marker)
        .expect("Missing ---- seperator");


    let metadata_bytes = &file_bytes[..marker_index];
    let encoded_bytes = &file_bytes[marker_index + marker.len()..];


    let metadata_str = String::from_utf8_lossy(metadata_bytes);
    let mut lines = metadata_str.lines();

    let bit_length: usize = lines.next().unwrap().parse()?;
    let mut huffman_map: HashMap<String, char> = HashMap::new();

    for line in lines {
        if let Some((char_str, bits)) = line.split_once(':') {
            let key = match char_str {
                "NEWLINE" => '\n',
                _ => char_str.chars().next().unwrap(),
            };
            huffman_map.insert(bits.to_string(), key);
        }
    }

    let mut bit_string = String::new();
    for byte in encoded_bytes {
        bit_string.push_str(&format!("{:08b}", byte));
    }
    bit_string.truncate(bit_length);


    let mut temp = String::new();
    let mut decoded = String::new();

    for bit in bit_string.chars() {
        temp.push(bit);
        if let Some(&ch) = huffman_map.get(&temp) {
            decoded.push(ch);
            temp.clear();
        } else {
            return Err("Failed to decode only bits".into());
        }
    }


    fs::write(output_file, decoded)?;

    Ok(())

}
