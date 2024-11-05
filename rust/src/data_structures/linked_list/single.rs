use std::fmt::Debug;

extern crate alloc;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
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
    pub fn new(value: T) -> Self {
        Self {
            head: Some(Box::new(Node::new(value))),
        }
    }
    pub fn push_front(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn push_back(&mut self, value: T) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node::new(value)));
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut current = &mut self.head;
        while current.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
        current.take().map(|node| node.value)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        let mut i = 0;
        while let Some(node) = current {
            if i == index {
                return Some(&node.value);
            }
            current = &node.next;
            i += 1;
        }
        None
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let mut current = &mut self.head;
        let mut i = 0;
        while let Some(node) = current {
            if i == index {
                return Some(&mut node.value);
            }
            current = &mut node.next;
            i += 1;
        }
        None
    }

    pub fn remove(&mut self, f: impl Fn(&T) -> bool) -> Option<T> {
        let mut current = &mut self.head;
        while current.is_some() {
            if (f)(&current.as_mut().unwrap().value) {
                let mut c = current.take().unwrap();
                let next = c.next.take();
                *current = next;
                return Some(c.value);
            }
            current = &mut current.as_mut().unwrap().next;
        }
        None
    }

    pub fn search(&self, f: impl Fn(&T) -> bool) -> Option<&T> {
        let mut current = &self.head;
        while let Some(node) = current {
            if (f)(&node.value) {
                return Some(&node.value);
            }
            current = &node.next;
        }
        None
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = &self.head;
        write!(f, "LinkedList: [")?;
        while let Some(node) = current {
            write!(f, "{:?} -> ", node.value)?;
            current = &node.next;
        }
        write!(f, "None]")
    }
}
