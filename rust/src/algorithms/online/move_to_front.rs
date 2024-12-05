use std::{mem::swap, ptr::NonNull, vec};

use crate::data_structures::linked_list::double::{LinkedList, Node};

pub fn swap_previous_next<T>(node: &mut NonNull<Node<T>>) -> usize {
    let mut cost = 0;
    let mut me: &mut Node<T> = unsafe { node.as_mut() };
    let mut previous = me.previous.as_mut();
    while let Some(previous_node) = previous {
        let previous_node = unsafe { previous_node.as_mut() };
        swap(&mut me.value, &mut previous_node.value);
        me = previous_node;
        previous = me.previous.as_mut();
        cost += 1;
    }
    cost
}

pub fn search_element_move_to_front<K: PartialEq, V: Copy>(
    list: &mut LinkedList<(K, V)>,
    key: K,
) -> (Option<V>, usize) {
    let mut cost = 0;
    let mut current = list.start_link();
    while let Some(node) = current {
        unsafe {
            if node.as_ref().value.0 == key {
                cost += swap_previous_next(node);
                let value = list.start_link().unwrap().as_ref().value.1;
                return (Some(value), cost);
            }
            current = &mut node.as_mut().next;
            cost += 1;
        }
    }
    (None, cost)
}

pub fn search_element_foresee<K: PartialEq, V: Copy>(
    list: &mut LinkedList<(K, V)>,
    key: K,
    future: K,
) -> (Option<V>, usize) {
    let mut cost = 0;
    let mut future_ok = false;
    let mut result = None;
    let mut current = list.start_link();
    while let Some(node) = current {
        unsafe {
            if node.as_ref().value.0 == key {
                result = Some(node.as_ref().value.1);
            }
            if !future_ok && node.as_ref().value.0 == future {
                cost += swap_previous_next(node);
                future_ok = true;
            }
            current = &mut node.as_mut().next;
            cost += 1;
        }
        if result.is_some() && future_ok {
            break;
        }
    }
    (result, cost)
}

pub fn calculate_futures<K: PartialEq, V: Copy>(
    list: &mut LinkedList<(K, V)>,
    futures: &[K],
) -> Vec<Option<usize>> {
    let mut result = vec![];
    for f in futures {
        let mut search_cost = 0;
        let mut current = list.start_link();
        while let Some(node) = current {
            unsafe {
                if node.as_ref().value.0 == *f {
                    result.push(Some(search_cost));
                    break;
                }
                current = &mut node.as_mut().next;
                search_cost += 1;
            }
        }
    }
    result
}
