use super::raw_vec::RawVecError;

pub mod chaining;
pub mod open_address;

#[derive(Debug)]
pub enum HashTableError {
    Memory(RawVecError),
    Full,
    Empty,
}

fn fix_capacity(mut capacity: usize) -> usize {
    let mut is_prime = false;
    while !is_prime {
        is_prime = true;
        if capacity % 2 == 0 {
            capacity += 1;
            is_prime = false;
            continue;
        }
        for i in (3..capacity / 2).step_by(2) {
            if capacity % i == 0 {
                is_prime = false;
                capacity += 1;
                break;
            }
        }
    }
    capacity
}
