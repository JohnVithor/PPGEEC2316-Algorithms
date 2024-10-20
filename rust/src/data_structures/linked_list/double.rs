use core::ptr::NonNull;

extern crate alloc;

type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub previous: Link<T>,
    pub next: Link<T>,
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
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

    fn into_non_null(self) -> NonNull<Node<T>> {
        unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(self))) }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    size: usize,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            front: None,
            back: None,
            size: 0,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new(value: T) -> Self {
        let new_node = Node::new(value).into_non_null();
        Self {
            front: Some(new_node),
            back: Some(new_node),
            size: 1,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node::new(value).into_non_null();
        if let Some(mut old) = self.front {
            unsafe {
                old.as_mut().previous = Some(new_node);
                new_node.as_mut().next = Some(old);
            }
        } else {
            self.back = Some(new_node);
        }
        self.front = Some(new_node);
        self.size += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let mut new_node = Node::new(value).into_non_null();
        if let Some(mut old) = self.back {
            unsafe {
                old.as_mut().next = Some(new_node);
                new_node.as_mut().previous = Some(old);
            }
        } else {
            self.front = Some(new_node);
        }
        self.back = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) {
        if let Some(mut first) = self.front {
            unsafe {
                let first = first.as_mut();
                if let Some(mut next) = first.next {
                    next.as_mut().previous = None;
                    self.front = Some(next);
                } else {
                    self.front = None;
                    self.back = None;
                }
            }
            self.size -= 1;
        }
    }

    pub fn pop_back(mut self) {
        if let Some(mut last) = self.back {
            unsafe {
                let last = last.as_mut();
                if let Some(mut previous) = last.previous {
                    previous.as_mut().next = None;
                    self.back = Some(previous);
                } else {
                    self.front = None;
                    self.back = None;
                }
            }
            self.size -= 1;
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.front.map(|node| unsafe { &node.as_ref().value })
    }

    pub fn back(&self) -> Option<&T> {
        self.back.map(|node| unsafe { &node.as_ref().value })
    }
}

impl<T: PartialEq> LinkedList<T> {
    pub fn search(&mut self, value: T) -> Option<&T> {
        let mut current = self.front;
        while let Some(node) = current {
            unsafe {
                if node.as_ref().value == value {
                    return Some(&node.as_ref().value);
                }
                current = node.as_ref().next;
            }
        }
        None
    }

    pub fn delete(self, value: T) {
        let mut current = self.front;
        while let Some(node) = current {
            unsafe {
                let node = node.as_ref();
                if node.value == value {
                    let prev = node.previous;
                    let next = node.next;
                    if let Some(mut prev) = prev {
                        prev.as_mut().next = next;
                    }
                    if let Some(mut next) = next {
                        next.as_mut().previous = prev;
                    }
                }
                current = node.next;
            }
        }
    }
}
