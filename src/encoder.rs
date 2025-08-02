use std::fs::{File, OpenOptions};
use std::error::Error;
use std::io::{BufReader, Read, BufWriter, Write};
use std::collections::{BinaryHeap, HashMap};
use std::process::exit;
use std::cmp::Reverse;

use crate::tree::{TreeNode, generate_code};
use crate::util::split_into_chunks;

pub fn encode(input_file: String, output_file: String) -> Result<(), Box<dyn Error>>{

    let mut contents = String::new();
    let file = File::open(input_file);
    match file {
        Ok(text) => {
            // Read file contents
            let mut reader = BufReader::new(text);

            reader.read_to_string(&mut contents)?;

            if contents.is_empty() {
                eprintln!("Empty file");
                exit(-1)
            }
        },
        Err(_) => {
            eprintln!("Can not open file")
        }
    }

    build_tree(contents, output_file)
}

fn build_tree(contents: String, output_file: String) -> Result<(), Box<dyn Error>> {
    // Prepare frequency table
    let mut freq_table = HashMap::new();

    let cloned_contents = contents.clone();

    for c in contents.chars() {
        let frequency = freq_table.entry(c).or_insert(0);
        *frequency += 1;
    }

    // Build the tree   
    let mut heap = BinaryHeap::new();
    for(char, freq) in freq_table {
        let node = TreeNode {
            character: Some(char),
            frequency: freq,
            left: None,
            right: None,
        };
        heap.push(Reverse(node));
    }

    while heap.len() > 1 {
        let Reverse(left_node) = heap.pop().unwrap();
        let Reverse(right_node) = heap.pop().unwrap();

        let parent = TreeNode {
            character: None,
            frequency: left_node.frequency + right_node.frequency,
            left: Some(Box::new(left_node)),
            right: Some(Box::new(right_node)),
        };
        heap.push(Reverse(parent));
    }
    generate_code_table(heap, &output_file, cloned_contents)
}

pub fn generate_code_table(mut heap: BinaryHeap<Reverse<TreeNode>>, output_file: &str, cloned_contents: String) -> Result <(), Box<dyn Error>> {
    // Generate the code table

    if let Some(Reverse(root)) = heap.pop() {
        let table = generate_code(&root);

        // Count entries and format each entry
        let num_of_chars = table.len();

        let mut output_lines = Vec::new();
        for (c, f) in &table {
            let key_str = match c {
                '\n' => "NEWLINE".to_string(),
                ' '  => continue, //"SPACE".to_string(),
                _    => c.to_string(),
            };

            let formatted = format!("{}:{}", key_str, f);
            output_lines.push(formatted);
        }

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(output_file)?;

        let mut writer = BufWriter::new(file);

        let _ = writeln!(writer, "{}", num_of_chars);

        for line in output_lines {
            writeln!(writer, "{}", line)?;
        }

        let _ = writer.write_all(b"----\n");

        // Encoding
        let mut growing_string = String::new();
        for ch in cloned_contents.chars() {
            match table.get(&ch) {
                Some(code) => growing_string.push_str(code),
                None => {
                    println!("Missing code for character: {:?}", ch);
                    panic!("Character not found in encoding table");
                }
            }
        }

        while growing_string.len() % 8 != 0 {
            growing_string.push('0');
        }

        let splitted = split_into_chunks(growing_string);
        let byte_vec: Vec<u8> = splitted
            .iter()
            .map(|chunk| u8::from_str_radix(chunk, 2).unwrap())
            .collect();
        writer.write_all(&byte_vec)?;
        return Ok(())
    } else {
        return Err("No output file provided".into());
    }
}
