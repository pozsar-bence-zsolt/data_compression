use std::{env, io::Result};

mod compress;
mod decompress;
mod codetable;

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    // args = vec!(
    //     "C:\\Users\\bence\\Documents\\University\\Data Compression\\Project\\files\\test_encode".to_string(),
    //     "C:\\Users\\bence\\Documents\\University\\Data Compression\\Project\\files\\t2".to_string()
    // );

    // args = vec!(
    //     "--decompress".to_string(),
    //     "C:\\Users\\bence\\Documents\\University\\Data Compression\\Project\\files\\t4".to_string(),
    //     "C:\\Users\\bence\\Documents\\University\\Data Compression\\Project\\files\\t4_decoded".to_string()
    // );

    let mut decompression = false;
    let mut in_path: String = String::new();
    let mut out_path: String = String::new();

    for arg in args {
        match arg.as_str() {
            "--decompress" => {
                decompression = true;
            }
            _ => {
                if in_path.is_empty() {
                    in_path = arg;
                } else if !in_path.is_empty() && out_path.is_empty() {
                    out_path = arg;
                }
            }
        }
    }

    if !decompression {
        compress::compress(in_path, out_path);
    } else {
        decompress::decompress(in_path, out_path);
    }

    Ok(())
}
