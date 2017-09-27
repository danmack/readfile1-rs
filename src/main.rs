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

    // a simple classic hex dump output

    let mut addr = 0;
    let	mut chars: [char; 16] = ['.'; 16];
    print_header();
    print!("{:08x} ", addr);

    for x in &file_buf {
        if (addr % 16 == 0) && (addr > 0) {
            print!("{:08x} ", addr);
        }
        print!("{:02x}", *x);
        if *x < 32 || *x > 125 {
            chars[addr%16] = '.';
        } else {
            chars[addr%16] = *x as char;
        }
        addr += 1;
        if addr%16 == 0 {
            print!("  |");
            for y in &chars {
                print!("{}", *y);
            }
            println!("|");
        } else {
            print!(" ");
        }
    }

    // deal with padding out the last line

    if (addr % 16) > 0 {
        let pad = 16-addr%16;
        for _ in 0..pad {
            print!("   ");
        }
        print!(" |");
        for z in 0..(addr%16) {
            print!("{}", chars[z]);
        }
        println!("|");
    }
    println!("\nhexdump of {} (0x{:x}) bytes completed.\n", addr, addr);
}

fn print_header() {
    println!("");
    println!("simple hexdump (dot <.> denotes unprintable char)\n");
    println!("87654321 00 11 22 33 44 55 66 77 88 99 aa bb cc dd ee ff  |0123456789abcdef|");
    println!("-------- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --  |----------------|");
}
