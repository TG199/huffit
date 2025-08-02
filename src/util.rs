pub fn split_into_chunks(mut bits: String) -> Vec<String> {
    // Splits bits into 8 bits chunks

    let remainder = bits.len() % 8;
    if remainder != 0 {
        let padding = 8 - remainder;
        bits.push_str(&"0".repeat(padding));
    }
    bits.as_bytes()
        .chunks(8)
        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        .collect()
}
