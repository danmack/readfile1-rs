use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let mut file_buf = Vec::new();

    match env::args().nth(1) {
        Some(file_name) => {
            print!("reading file name: {}", file_name);
            match fs::File::open(&file_name) {
                Ok(mut fd) => {
                    match fd.read_to_end(&mut file_buf) {
                        Ok(bytes) => {
                            println!(".. {} bytes read.", bytes);
                        }
                        Err(e) => {
                            panic!("error reading the file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    panic!("failed to open file: {}", e);
                }
            }
        }
        None => {
            panic!("no file argument present");
        }
    }

    let mut c = 1;
    for x in &file_buf {
        print!("{:02X}", x);
        if c%16 == 0 {
            println!("");
        } else {
            print!(" ");
        }
        c += 1;
    }
}
