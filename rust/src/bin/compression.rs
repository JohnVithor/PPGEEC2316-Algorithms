use algorithms::algorithms::huffman_edited::{compress, decompress};

fn main() {
    let original = b"this is a test string for huffman compression";
    println!("Original: {:?}", String::from_utf8_lossy(original));
    let compressed = compress(original);
    let decompressed = decompress(&compressed);
    println!(
        "Original: {:?}",
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
}
