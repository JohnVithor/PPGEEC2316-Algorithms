use std::{hash::Hash, mem::swap, ptr::NonNull};

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

pub fn move_to_front<K: PartialEq, V: Copy>(
    list: &mut LinkedList<(K, V)>,
    key: &K,
) -> (Option<V>, usize) {
    let mut cost = 0;
    let mut current = list.start_link();
    while let Some(node) = current {
        unsafe {
            if node.as_ref().value.0 == *key {
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

/// O pior caso do algoritmo move-to-front é quando o próximo elemento a ser acessado é o último elemento na organização atual da lista
/// Nesse caso, o algoritmo move-to-front terá que percorrer toda a lista para encontrar o elemento
/// E em seguida terá que percorrer a lista novamente para trazer o elemento para frente
/// Enquanto que o algoritmo foresse nessa situação simplesmente não traria o elemento para frente
pub fn foresee_on_move_to_front_worst_case<K: Eq + Hash, V: Copy>(
    search_list: &mut LinkedList<(K, V)>,
    search_sequence: &[K],
) -> usize {
    // Fórmula analítica do custo nessa situação: (1+search_list)*search_list/2*search_sequence/search_list
    // Abaixo: Simulação do custo
    let mut acc_cost = 0;
    for key in search_sequence {
        let mut current = search_list.start_link();
        acc_cost += 1;
        while let Some(node) = current {
            unsafe {
                if node.as_ref().value.0 == *key {
                    break;
                }
                current = &mut node.as_mut().next;
                acc_cost += 1;
            }
        }
    }
    acc_cost
}

pub fn move_to_front_simulation_on_list<K: Eq + Hash, V: Copy>(
    search_list: &mut LinkedList<(K, V)>,
    search_sequence: &[K],
) -> usize {
    // Fórmula analítica do custo no pior caso: (search_sequence+search_sequence-1)*search_list
    // Abaixo: Simulação do custo
    let mut acc_cost = 0;
    for key in search_sequence {
        let (_, cost) = move_to_front(search_list, key);
        acc_cost += cost;
    }
    acc_cost
}
