use std::{fs::File, io::{BufReader, Write}};

use bitstream_io::{BitRead, BitReader, LittleEndian};

use crate::codetable::CodeTable;

pub fn decompress(in_path: String, out_path: String) {
    let compressed_file = match File::open(in_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open input file: {}", e);
            return;
        }
    };

    let mut file = match File::create(out_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create output file: {}", e);
            return;
        }
    };
    let mut dictionary: CodeTable = CodeTable::create();

    let buf_reader = BufReader::new(compressed_file);
    let mut bit_reader: BitReader<BufReader<File>, LittleEndian> = BitReader::<_, LittleEndian>::new(buf_reader);

    let mut prev_vec: Vec<u8> = Vec::new();
    let mut current_vec: Vec<u8> = Vec::new();

    let mut write_buffer: Vec<u8> = Vec::new();

    loop {
        let read_value = match bit_reader.read::<16, u16>() {
            Ok(bit_read) => bit_read,
            Err(_) => break,
        };

        match dictionary.get_key(read_value) {
            Some(key) => {
                current_vec = key.clone();
                write_buffer.extend_from_slice(&key);
                

                if !prev_vec.is_empty() {
                    let mut new_key = prev_vec.clone();
                    new_key.push(current_vec[0]);
                    dictionary.put_value(&new_key);
                }

                prev_vec = current_vec.clone();
            }
            None => {
                if prev_vec.is_empty() {
                    current_vec.push(read_value as u8);
                    dictionary.put_value(&current_vec);
                    write_buffer.extend_from_slice(&current_vec);
                } else {
                    let mut new_key = prev_vec.clone();
                    new_key.push(prev_vec[0]);
                    current_vec = new_key.clone();
                    dictionary.put_value(&current_vec);
                    write_buffer.extend_from_slice(&current_vec.clone());
                }

                prev_vec = current_vec.clone();
            }
        }

        if write_buffer.len() >= 8192 {
            write_output(&mut file, &write_buffer);
            write_buffer.clear();
        }
    }

    if !write_buffer.is_empty() {
        write_output(&mut file, &write_buffer);
    }

}

fn write_output(file: &mut File, decode: &Vec<u8>) {
    // println!("File write: {:?}", decode);
    match file.write(&decode) {
        Ok(_) => {},
        Err(error) => panic!("{:?}", error)
    }
}