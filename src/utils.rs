use std::{fs, io::{self, Read}};

pub fn read_bytes(path: &str) -> Vec<(u8, u8)> {
    let mut instruction_data: Vec<(u8, u8)> = Vec::new();

    let file = match fs::File::open(path){
        Ok(f) => f,
        Err(e) => {
            panic!("Error while reading file {}: {}", path.to_string(), e.to_string());
        }
    };

    let reader = io::BufReader::new(file);
    let mut buffer: (u8, u8) = (0x0, 0x0);
    for (index, byte_result) in reader.bytes().enumerate() {
        let current_byte = match byte_result {
            Ok(b) => b,
            Err(e) => {
                panic!("An error occured while reading the program file: {}", e.to_string());
            }
        };
        if index % 2 != 0 {
            buffer.0 = current_byte;
            instruction_data.push(buffer);
        } else {
            buffer.1 = current_byte;
        }
    }

    instruction_data
}