use crate::data_structures::linked_list::double::{LinkedList, Node};

use super::{Interval, IntervalManager};

pub struct LinkedListIntervalManager {
    data: LinkedList<Interval>,
}

impl IntervalManager for LinkedListIntervalManager {
    fn add_user(&mut self, user: i32, size: usize) -> Option<&Interval> {
        let mut current = self.data.start_link();
        while let Some(node) = current {
            unsafe {
                if node.as_ref().value.user_id.is_none()
                    && node.as_ref().value.end - node.as_ref().value.start >= size
                {
                    let interval = Interval {
                        start: node.as_ref().value.start + size,
                        end: node.as_ref().value.end,
                        user_id: None,
                    };
                    node.as_mut().value.end = node.as_ref().value.start + size;
                    node.as_mut().value.user_id = Some(user);
                    let mut new_node = Node::new(interval).into_non_null();
                    new_node.as_mut().previous = Some(*node);
                    new_node.as_mut().next = node.as_ref().next;
                    node.as_mut().next = Some(new_node);
                    if let Some(mut next) = new_node.as_ref().next {
                        next.as_mut().previous = Some(new_node);
                    }
                    return Some(&node.as_ref().value);
                }
                current = &mut node.as_mut().next;
            }
        }
        None
    }

    fn remove_user(&mut self, user: i32) {
        let mut current = self.data.start_link();
        while let Some(node) = current {
            unsafe {
                if node.as_ref().value.user_id == Some(user) {
                    node.as_mut().value.user_id = None;
                    if let Some(mut next) = node.as_ref().next {
                        if let Some(mut previous) = node.as_ref().previous {
                            if previous.as_ref().value.user_id.is_none() {
                                previous.as_mut().value.end = next.as_ref().value.end;
                                previous.as_mut().next = next.as_ref().next;
                                if let Some(mut next_next) = next.as_ref().next {
                                    next_next.as_mut().previous = Some(previous);
                                }
                            }
                        }
                    }
                    return;
                }
                current = &mut node.as_mut().next;
            }
        }
    }

    fn get_interval(&self, user: i32) -> Option<&Interval> {
        todo!()
    }
}
