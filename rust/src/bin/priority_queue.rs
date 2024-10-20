use algorithms::data_structures::binary_heap::binary_heap_explicit_key::BinaryHeap;

fn main() {
    let queue_objects = (1..=10).collect::<Vec<i32>>();
    let queue_keys = (1..=10).collect::<Vec<i32>>();
    let mut priority_queue = BinaryHeap::create(queue_objects.iter().collect(), queue_keys);

    println!("{:?}", priority_queue.data);
    let first = priority_queue.extract_max();
    println!("{:?}", first.unwrap());
    println!("{:?}", priority_queue.data);
    priority_queue.increase_key(&&queue_objects[0], 100);
    println!("{:?}", priority_queue.data);
    priority_queue.insert(&queue_objects[0], 100);
}
