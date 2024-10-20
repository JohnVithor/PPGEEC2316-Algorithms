use algorithms::data_structures::{linked_list::single::LinkedList, queue::Queue, stack::Stack};

fn main() {
    let mut stack = Stack::new(3).unwrap();
    stack.push(0).unwrap();
    stack.push(5).unwrap();
    stack.push(8).unwrap();
    println!("{:?}", stack);
    let popped = stack.pop();
    println!("{:?}", popped);
    println!("{:?}", stack);
    let peeked = stack.pop();
    println!("{:?}", peeked);
    println!("{:?}", stack);

    let mut queue = Queue::new(10).unwrap();
    queue.enqueue(0).unwrap();
    queue.enqueue(5).unwrap();
    queue.enqueue(8).unwrap();
    println!("{:?}", queue);
    let popped = queue.dequeue();
    println!("{:?}", popped);
    println!("{:?}", queue);
    let peeked = queue.dequeue();
    println!("{:?}", peeked);
    println!("{:?}", queue);
    queue.enqueue(2).unwrap();
    println!("{:?}", queue);

    let mut list = LinkedList::default();
    list.push_front(0);
    list.push_front(5);
    list.push_front(8);
    println!("{:?}", list);
    let popped = list.pop_front();
    println!("{:?}", popped);
    println!("{:?}", list);
    let peeked = list.pop_front();
    println!("{:?}", peeked);
    println!("{:?}", list);
    list.push_front(2);
    println!("{:?}", list);
}
