use algorithms::linked_list::double::LinkedList;

fn main() {
    let mut list = LinkedList::new(9);
    list.push_back(3);
    println!("{:?}", list.front());
    list.pop_front();
    println!("{:?}", list);
}
