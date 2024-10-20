use crate::data_structures::binary_heap::binary_heap_implicit_key::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    freq: usize,
    byte: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(freq: usize, byte: Option<u8>) -> Self {
        Node {
            freq,
            byte,
            left: None,
            right: None,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for Node {}

#[derive(Debug)]
pub struct CompressedData {
    pub data: Vec<u8>,
    padding_bits: u8,
    encoding_map: HashMap<u8, Vec<bool>>,
}

fn build_frequency_map(bytes: &[u8]) -> HashMap<u8, usize> {
    let mut freq_map = HashMap::new();
    for &byte in bytes {
        *freq_map.entry(byte).or_insert(0) += 1;
    }
    freq_map
}

fn build_huffman_tree(freq_map: HashMap<u8, usize>) -> Option<Node> {
    let mut heap = BinaryHeap::new(
        freq_map
            .into_iter()
            .map(|(v, k)| Node::new(k, Some(v)))
            .collect(),
    );

    while heap.data.len() > 1 {
        let left = heap.extract_max()?;
        let right = heap.extract_max()?;

        let mut parent = Node::new(left.freq + right.freq, None);
        parent.left = Some(Box::new(left));
        parent.right = Some(Box::new(right));

        heap.insert(parent);
    }

    heap.extract_max()
}

fn build_encoding_map(root: &Node, current_code: Vec<bool>, map: &mut HashMap<u8, Vec<bool>>) {
    if let Some(byte) = root.byte {
        map.insert(byte, current_code);
        return;
    }

    if let Some(left) = &root.left {
        let mut left_code = current_code.clone();
        left_code.push(false);
        build_encoding_map(left, left_code, map);
    }

    if let Some(right) = &root.right {
        let mut right_code = current_code;
        right_code.push(true);
        build_encoding_map(right, right_code, map);
    }
}

pub fn compress(bytes: &[u8]) -> CompressedData {
    let freq_map = build_frequency_map(bytes);
    let tree = build_huffman_tree(freq_map).unwrap();

    let mut encoding_map = HashMap::new();
    build_encoding_map(&tree, Vec::new(), &mut encoding_map);

    let mut compressed = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    for &byte in bytes {
        let code = encoding_map.get(&byte).unwrap();
        for &bit in code {
            if bit {
                current_byte |= 1 << (7 - bit_count);
            }
            bit_count += 1;

            if bit_count == 8 {
                compressed.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }
    }

    let padding_bits = if bit_count > 0 {
        compressed.push(current_byte);
        8 - bit_count
    } else {
        0
    };

    CompressedData {
        data: compressed,
        padding_bits,
        encoding_map,
    }
}

pub fn decompress(compressed: &CompressedData) -> Vec<u8> {
    let mut reverse_map = HashMap::new();
    for (&byte, code) in &compressed.encoding_map {
        reverse_map.insert(code.clone(), byte);
    }

    let mut result = Vec::new();
    let mut current_code = Vec::new();

    let last_byte_bits = if compressed.data.is_empty() {
        0
    } else {
        8 - compressed.padding_bits
    };

    for (i, &byte) in compressed.data.iter().enumerate() {
        let bits_to_read = if i == compressed.data.len() - 1 && compressed.padding_bits > 0 {
            last_byte_bits
        } else {
            8
        };

        for bit_pos in 0..bits_to_read {
            let bit = (byte & (1 << (7 - bit_pos))) != 0;
            current_code.push(bit);

            if let Some(&decoded_byte) = reverse_map.get(&current_code) {
                result.push(decoded_byte);
                current_code.clear();
            }
        }
    }

    result
}
