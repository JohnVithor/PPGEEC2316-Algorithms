use std::env::args;

use algorithms::algorithms::huffman_explicit_key::{compress, decompress};

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
    println!(
        "Mensagem Original: {:?}",
        String::from_utf8_lossy(&original)
    );
    let compressed = compress(&original);
    println!("Mensagem comprimida: {:?}", compressed.data.as_slice());
    let decompressed = decompress(&compressed);
    println!(
        "Mensagem descomprimida: {:?}",
        String::from_utf8_lossy(decompressed.as_slice())
    );

    println!("Tamanho Original: {} bytes", original.len());
    println!("Tamanho Comprimido: {} bytes", compressed.data.len());
    println!(
        "Taxa de Compressão: {:.2}%",
        (1.0 - (compressed.data.len() as f64 / original.len() as f64)) * 100.0
    );
    println!("Descompressão bem sucedida: {}", original == decompressed);
    Ok(())
}
