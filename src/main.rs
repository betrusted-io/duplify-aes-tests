use std::fs::File;
use std::io::{BufWriter, Write};

use blobby::Blob3Iterator;

fn print_header<U: Write>(out: &mut U) -> std::io::Result<()> {
    let s = r####"#![allow(dead_code)]
// from https://www.cosic.esat.kuleuven.be/nessie/testvectors/
pub(crate) struct Aes128Test {
    pub key: [u8; 16],
    pub pt: [u8; 16],
    pub ct: [u8; 16],
}
pub(crate) struct Aes256Test {
    pub key: [u8; 32],
    pub pt: [u8; 16],
    pub ct: [u8; 16],
}"####;
    out.write_all(s.as_bytes())
}
//const AES_128_TESTS: [AesTest; 1] =  [AesTest{ key: [0; 16], pt: [0; 16], ct: [0; 16]}];

fn main() -> std::io::Result<()> {
    let file = File::create("aes128tests.rs")?;
    let mut writer = BufWriter::new(&file);
    print_header(&mut writer)?;

    let data = include_bytes!(concat!("data/", "aes128", ".blb"));
    let num_rows = Blob3Iterator::new(data).unwrap().count();

    write!(writer, "pub(crate) const AES_128_TESTS: [Aes128Test; {}] = [\n", num_rows)?;
    for (_i, row) in Blob3Iterator::new(data).unwrap().enumerate() {
        let [key, pt, ct] = row.unwrap();
        write!(writer, "    Aes128Test {{\n        key: [")?;
        for &byte in key.iter() {
            write!(writer, "0x{:02x},", byte)?;
        }
        write!(writer, "],\n        pt: [")?;
        for &byte in pt.iter() {
            write!(writer, "0x{:02x},", byte)?;
        }
        write!(writer, "],\n        ct: [")?;
        for &byte in ct.iter() {
            write!(writer, "0x{:02x},", byte)?;
        }
        write!(writer, "]\n    }},\n")?;
    }
    write!(writer, "];\n")?;

    // because we're too lazy to macro-ize or parameterize run-once code.

    let file = File::create("aes256tests.rs")?;
    let mut writer = BufWriter::new(&file);
    print_header(&mut writer)?;

    let data = include_bytes!(concat!("data/", "aes256", ".blb"));
    let num_rows = Blob3Iterator::new(data).unwrap().count();

    write!(writer, "pub(crate) const AES_256_TESTS: [Aes256Test; {}] = [\n", num_rows)?;
    for (_i, row) in Blob3Iterator::new(data).unwrap().enumerate() {
        let [key, pt, ct] = row.unwrap();
        write!(writer, "    Aes256Test {{\n        key: [")?;
        for &byte in key.iter() {
            write!(writer, "0x{:02x},", byte)?;
        }
        write!(writer, "],\n        pt: [")?;
        for &byte in pt.iter() {
            write!(writer, "0x{:02x},", byte)?;
        }
        write!(writer, "],\n        ct: [")?;
        for &byte in ct.iter() {
            write!(writer, "0x{:02x},", byte)?;
        }
        write!(writer, "]\n    }},\n")?;
    }
    write!(writer, "];\n")?;

    Ok(())
}
