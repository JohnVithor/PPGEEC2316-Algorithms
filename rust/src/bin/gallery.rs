use std::collections::{vec_deque::VecDeque, LinkedList};

use algorithms::data_structures::trees::binary_tree::BinaryTree;

struct Photo {
    name: String,
}

fn main() {
    // let photos_array: Vec<Photo> = Vec::new();
    // let photos_stack: VecDeque<Photo> = VecDeque::new();
    // let photos_queue: VecDeque<Photo> = VecDeque::new();
    // let photos_linked_list: LinkedList<Photo> = LinkedList::new();
    // let photos_binary_tree: BinaryTree<String, Photo> = BinaryTree::new();
    let mut test = BinaryTree::new();
    test.insert(2, 20);
    test.insert(1, 10);
    test.insert(3, 30);
    test.insert(4, 40);
    println!("{:?}", test.search(1));
    println!("{:?}", test.search(2));
    println!("{:?}", test.search(3));
    println!("{:?}", test.search(4));
    let r = test.remove(2);
    println!("removed: {:?}", r);
    println!("{:?}", test.search(1));
    println!("{:?}", test.search(2));
    println!("{:?}", test.search(3));
    println!("{:?}", test.search(4));
}
