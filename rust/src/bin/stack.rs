use algorithms::stack::Stack;

#[derive(Debug)]
struct Test;

fn main() {
    let mut stack = Stack::new(10).unwrap();
    stack.push(Test).unwrap();
    stack.push(Test).unwrap();
    stack.push(Test).unwrap();
    println!("{:?}", stack);
    let popped = stack.pop();
    println!("{:?}", popped);
    println!("{:?}", stack);
    let peeked = stack.pop();
    println!("{:?}", peeked);
    println!("{:?}", stack);
}
