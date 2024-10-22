use std::env::args;

use algorithms::algorithms::huffman_edited::{compress, decompress};

fn main() -> Result<(), ()> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Uso: {} <string>", args[0],);
        return Err(());
    }
    let text = &args[1];

    if text.is_empty() {
        println!("Uso: {} <string>", args[0],);
        return Err(());
    }

    let original = text.bytes().collect::<Vec<u8>>();
    println!("Original: {:?}", String::from_utf8_lossy(&original));
    let compressed = compress(&original);
    let decompressed = decompress(&compressed);
    println!(
        "decompressed: {:?}",
        String::from_utf8_lossy(decompressed.as_slice())
    );

    println!("Original size: {} bytes", original.len());
    println!("Compressed size: {} bytes", compressed.data.len());
    println!(
        "Compression ratio: {:.2}%",
        (1.0 - (compressed.data.len() as f64 / original.len() as f64)) * 100.0
    );
    println!(
        "Successfully decompressed: {}",
        original == decompressed.as_slice()
    );
    Ok(())
}
