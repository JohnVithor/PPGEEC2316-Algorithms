extern crate alloc;

use alloc::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

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
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> LinkedList<T> {
    pub fn preprend(&self, value: T) -> LinkedList<T> {
        LinkedList {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> LinkedList<T> {
        LinkedList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}
