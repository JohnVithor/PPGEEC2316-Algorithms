use std::{env::args, fmt::Debug, time::Instant};

use algorithms::{
    algorithms::online::move_to_front::{search_element_foresee, search_element_move_to_front},
    data_structures::linked_list::double::LinkedList,
};

fn print_list<T: Debug>(list: &mut LinkedList<T>) {
    let mut current = list.start_link();
    while let Some(node) = current {
        unsafe {
            print!("{:?} ", node.as_ref().value);
            current = &mut node.as_mut().next;
        }
    }
    println!();
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        println!("Uso: {} <n> <seed>", args[0]);
        return Err(());
    }
    let size: usize = args[1].parse().unwrap();
    let seed: i64 = args[2].parse().unwrap();

    if size < 2 || seed < 0 {
        println!("Uso: {} <n> <seed>", args[0],);
        return Err(());
    }
    fastrand::seed(seed as u64);
    let mut list1 = LinkedList::default();
    let mut list2 = LinkedList::default();
    {
        let mut data: Vec<usize> = (0..size).collect();
        for i in 0..size {
            data.swap(i, fastrand::usize(i..size));
            list1.push_back((i, data[i]));
            list2.push_back((i, data[i]));
        }
    }

    println!("Data size: {}", size);

    let start = Instant::now();
    let r1 = search_element_move_to_front(&mut list1, 5);
    let time_spent_rand = start.elapsed().as_secs_f64();
    println!("Tempo gasto: {}", time_spent_rand);
    println!("Resultado: {:?}", r1);
    print_list(&mut list1);

    let start = Instant::now();
    let r1 = search_element_move_to_front(&mut list1, 5);
    let time_spent_rand = start.elapsed().as_secs_f64();
    println!("Tempo gasto: {}", time_spent_rand);
    println!("Resultado: {:?}", r1);
    print_list(&mut list1);

    let start = Instant::now();
    let r1 = search_element_foresee(&mut list2, 5, 5);
    let time_spent_rand = start.elapsed().as_secs_f64();
    println!("Tempo gasto: {}", time_spent_rand);
    println!("Resultado: {:?}", r1);
    print_list(&mut list2);

    let start = Instant::now();
    let r1 = search_element_foresee(&mut list2, 5, 5);
    let time_spent_rand = start.elapsed().as_secs_f64();
    println!("Tempo gasto: {}", time_spent_rand);
    println!("Resultado: {:?}", r1);
    print_list(&mut list2);

    Ok(())
}
