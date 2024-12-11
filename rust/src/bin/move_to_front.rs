use std::{env::args, time::Instant};

use algorithms::{
    algorithms::online::move_to_front::{
        foresee_on_move_to_front_worst_case, move_to_front_simulation_on_list,
    },
    data_structures::linked_list::double::LinkedList,
};

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

    let access_sequence: Vec<usize> = (0..size).rev().collect();

    let start = Instant::now();
    let mtf_cost = move_to_front_simulation_on_list(&mut list1, &access_sequence);
    let time_spent_mtf = start.elapsed().as_secs_f64();

    let start = Instant::now();
    let foresee_cost = foresee_on_move_to_front_worst_case(&mut list2, &access_sequence);
    let time_spent_foresee = start.elapsed().as_secs_f64();

    println!("{size},{mtf_cost},{foresee_cost},{time_spent_mtf},{time_spent_foresee}");

    Ok(())
}
