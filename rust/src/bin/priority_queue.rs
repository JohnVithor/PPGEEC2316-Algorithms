use algorithms::BinaryHeap;

fn main() {
    let queue_objects = (1..=10).collect::<Vec<i32>>();
    let queue_keys = (1..=10).map(|x| x as f32).collect::<Vec<f32>>();
    let mut priority_queue = BinaryHeap::new(queue_objects.iter().collect(), queue_keys);

    println!("{:?}", priority_queue.data);
    let first = priority_queue.extract_max();
    println!("{:?}", first.unwrap());
    println!("{:?}", priority_queue.data);
    priority_queue.increase_key(&&queue_objects[0], 100.0);
    println!("{:?}", priority_queue.data);
    priority_queue.insert(&queue_objects[0], 100.0);
}
