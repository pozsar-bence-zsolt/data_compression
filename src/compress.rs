use std::io::Write;
use std::{fs::File, io::BufReader};

use bitstream_io::{BitRead, BitReader, LittleEndian};

use crate::codetable::{CodeTable, LZWBase};

pub fn compress(in_path: String, out_path: String) {
    let file = match File::open(in_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open input file: {}", e);
            return;
        },
    };

    let mut compressed_file = match File::create(out_path){
        Ok(file) => file,
        Err(_) => {
            println!("Output file could not be created");
            return;
        }
    };

    let mut dictionary: CodeTable = CodeTable::create();

    let buf_reader = BufReader::new(file);
    let mut bit_reader = BitReader::<_, LittleEndian>::new(buf_reader);

    let mut read_vec: Vec<u8> = Vec::new();
    let mut prev_vec: Vec<u8> = Vec::new();
    let mut bit_read: u8 = 0;
    let mut need_read: bool = true;
    let mut end_file: bool = false;
    let mut prev_value: u32 = 0;
    loop {
        if need_read {
            bit_read = match bit_reader.read::<8, u8>() {
                Ok(bit_read) => {
                    prev_vec = read_vec.clone();
                    read_vec.push(bit_read);
                    bit_read
                },
                Err(_) => {
                    end_file = true;
                    0
                }
            };
        }

        if !end_file {
            match dictionary.get_value(&read_vec) {
                Some(value) => {
                    need_read = true;
                    prev_value = value;
                },
                None => {
                    dictionary.put_value(&read_vec);
                    write_output(&mut compressed_file, &prev_value);
                    prev_vec.clear();
                    read_vec.clear();
                    read_vec.push(bit_read);
                    need_read = false;
                }
            }
        } else {
            write_output(&mut compressed_file, &prev_value);
            break;
        }
    }
}

fn write_output(file: &mut File, encode: &u32) {
    let buffer_bytes: [u8; 4] = encode.to_le_bytes();
    let buffer_bytes_trunc: [u8; 3] = buffer_bytes[..3].try_into().unwrap();

    match file.write(&buffer_bytes_trunc) {
        Ok(_) => {},
        Err(error) => {
            println!("{:?}", error)
        }
    }
}