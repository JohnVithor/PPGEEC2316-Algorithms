extern crate alloc;

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub value: T,
    pub next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            next: None,
        }
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.next.take(),
        };
        self.next = Some(Box::new(new_node));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.next.take().map(|node| {
            self.next = node.next;
            node.value
        })
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> LinkedList<T> {
    pub fn push_front(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }
}
