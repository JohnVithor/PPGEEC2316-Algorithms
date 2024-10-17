extern crate alloc;

use alloc::rc::Rc;

pub struct Node<T> {
    pub value: T,
    pub previous: Option<Rc<Node<T>>>,
    pub next: Option<Rc<Node<T>>>,
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            previous: None,
            next: None,
        }
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            previous: None,
            next: None,
        }
    }

    pub fn append(mut self, value: T) {
        let mut new_node = Node {
            value,
            next: self.next.take(),
            previous: None,
        };
        new_node.previous = Some(Rc::new(self));
        self.next = Some(Rc::new(new_node));
    }

    pub fn preprend(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.next.take(),
        };
        self.next = Some(Rc::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.next.take().map(|node| {
            self.next = node.next;
            node.value
        })
    }
}
