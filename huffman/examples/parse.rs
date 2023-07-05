//! The example will print out the source code for the ENCODE_TABLE constant
//! which is provided by this crate through the `encode::table` module.

extern crate httlib_huffman;

use std::{fs, path::Path};

use httlib_huffman::parser::parse;

fn main() {
    let path = Path::new("assets/hpack-huffman.txt");
    let data = fs::read_to_string(path).expect("Can't read file.");
    let codings = parse(&data);

    println!("");
    println!("/// This is a static Huffman table built from the codes found in the official");
    println!("/// HPACK specification (Appendix B).");
    println!("pub const ENCODE_TABLE: [(u8, u32); 257] = [ // (length, msb)");
    for (i, coding) in codings.iter().enumerate() {
        if i > 0 {
            println!(",")
        }
        print!("  ({}, 0x{:02x})", coding.0, coding.1);
    }
    println!("");
    println!("];");
    println!("");
}
