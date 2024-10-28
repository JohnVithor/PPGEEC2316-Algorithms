use std::cmp::Ordering;

struct Node<K, T> {
    key: K,
    value: T,
    left: Option<Box<Node<K, T>>>,
    right: Option<Box<Node<K, T>>>,
}

pub enum NodeOperation {
    Updated,
    Inserted,
    Error,
}

impl<K: PartialOrd, T> Node<K, T> {
    fn new(key: K, value: T) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, key: K, value: T) -> NodeOperation {
        if let Some(ord) = self.key.partial_cmp(&key) {
            match ord {
                Ordering::Equal => {
                    self.value = value;
                    NodeOperation::Updated
                }
                Ordering::Less => match self.left {
                    Some(ref mut left) => left.insert(key, value),
                    None => {
                        self.left = Some(Box::new(Node::new(key, value)));
                        NodeOperation::Inserted
                    }
                },
                Ordering::Greater => match self.right {
                    Some(ref mut right) => right.insert(key, value),
                    None => {
                        self.right = Some(Box::new(Node::new(key, value)));
                        NodeOperation::Inserted
                    }
                },
            }
        } else {
            NodeOperation::Error
        }
    }

    fn delete(mut self: Box<Node<K, T>>, target: &K) -> (Option<Box<Node<K, T>>>, Option<T>) {
        if target < &self.key {
            if let Some(left) = self.left.take() {
                let (new_left, result) = left.delete(target);
                self.left = new_left;
                return (self.left, result);
            }
            return (Some(self), None);
        }

        if target > &self.key {
            if let Some(right) = self.right.take() {
                let (new_right, result) = right.delete(target);
                self.right = new_right;
                return (self.right, result);
            }
            return (Some(self), None);
        }

        let result = Some(self.value);
        match (self.left.take(), self.right.take()) {
            (None, None) => (None, result),
            (Some(left), None) => (Some(left), result),
            (None, Some(right)) => (Some(right), result),
            (Some(mut left), Some(right)) => {
                if let Some(mut rightmost) = left.rightmost_child() {
                    rightmost.left = Some(left);
                    rightmost.right = Some(right);
                    (Some(rightmost), result)
                } else {
                    left.right = Some(right);
                    (Some(left), result)
                }
            }
        }
    }

    fn rightmost_child(&mut self) -> Option<Box<Node<K, T>>> {
        match self.right {
            Some(ref mut right) => {
                if let Some(t) = right.rightmost_child() {
                    Some(t)
                } else {
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = r.left.take();
                    }
                    r
                }
            }
            None => None,
        }
    }
}

pub struct BinaryTree<K, T> {
    root: Option<Box<Node<K, T>>>,
}

impl<K, T> Default for BinaryTree<K, T> {
    fn default() -> Self {
        BinaryTree { root: None }
    }
}

impl<K: PartialOrd, T> BinaryTree<K, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_with(key: K, value: T) -> Self {
        BinaryTree {
            root: Some(Box::new(Node::new(key, value))),
        }
    }

    pub fn insert(&mut self, key: K, value: T) -> NodeOperation {
        match self.root {
            Some(ref mut root) => root.insert(key, value),
            None => {
                self.root = Some(Box::new(Node::new(key, value)));
                NodeOperation::Inserted
            }
        }
    }

    pub fn search(&self, key: K) -> Option<&T> {
        let mut current = &self.root;
        while let Some(ref node) = *current {
            if let Some(ord) = node.key.partial_cmp(&key) {
                match ord {
                    Ordering::Equal => return Some(&node.value),
                    Ordering::Less => current = &node.left,
                    Ordering::Greater => current = &node.right,
                }
            } else {
                return None;
            }
        }
        None
    }

    pub fn remove(&mut self, key: K) -> Option<T> {
        if let Some(root) = self.root.take() {
            let (new_root, result) = root.delete(&key);
            self.root = new_root;
            return result;
        }
        None
    }
}
