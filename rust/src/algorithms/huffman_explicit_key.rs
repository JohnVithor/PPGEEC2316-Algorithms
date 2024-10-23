use crate::data_structures::binary_heap::binary_heap_explicit_key::BinaryHeap;
use std::{collections::HashMap, vec};

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct CompressedData {
    pub data: Vec<u8>,
    padding_bits: u8,
    encoding_map: [Vec<bool>; 256],
}

fn build_frequency_map(bytes: &[u8]) -> [usize; 256] {
    let mut freq_map = [0; 256];
    for &byte in bytes {
        freq_map[byte as usize] += 1;
    }
    freq_map
}

fn build_huffman_tree(freq_map: [usize; 256]) -> Option<Node> {
    let mut heap = BinaryHeap::new(
        freq_map
            .into_iter()
            .enumerate()
            .filter(|(_, v)| *v > 0)
            .map(|(k, v)| (v, Node::new(v, Some(k as u8))))
            .collect(),
    );

    while heap.data.len() > 1 {
        let left = heap.extract_max()?;
        let right = heap.extract_max()?;
        let freq = left.freq + right.freq;
        let mut parent = Node::new(freq, None);
        parent.left = Some(Box::new(left));
        parent.right = Some(Box::new(right));

        heap.insert(parent, freq);
    }

    heap.extract_max()
}

fn build_encoding_map(root: &Node) -> [Vec<bool>; 256] {
    const ARRAY_REPEAT_VALUE: Vec<bool> = Vec::new();
    let mut map = [ARRAY_REPEAT_VALUE; 256];
    if root.left.is_none() && root.right.is_none() {
        map[root.byte.unwrap() as usize] = vec![false];
        return map;
    }
    let mut stack = Vec::new();
    stack.push((root, vec![]));

    while let Some((node, current_code)) = stack.pop() {
        if let Some(byte) = node.byte {
            map[byte as usize] = current_code;
        } else {
            if let Some(right) = &node.right {
                let mut right_code = current_code.clone();
                right_code.push(true);
                stack.push((right, right_code));
            }
            if let Some(left) = &node.left {
                let mut left_code = current_code;
                left_code.push(false);
                stack.push((left, left_code));
            }
        }
    }
    map
}

pub fn compress(bytes: &[u8]) -> CompressedData {
    let freq_map = build_frequency_map(bytes);
    let tree = build_huffman_tree(freq_map).unwrap();
    let encoding_map = build_encoding_map(&tree);

    let mut compressed = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    for &byte in bytes {
        let code = &encoding_map[byte as usize];
        for bit in code {
            if *bit {
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
    for (byte, code) in compressed.encoding_map.iter().enumerate() {
        reverse_map.insert(code.clone(), byte as u8);
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
