use super::raw_vec::RawVecError;

pub mod chaining;
pub mod open_address;

#[derive(Debug)]
pub enum HashTableError {
    Memory(RawVecError),
    Full,
    Empty,
}
