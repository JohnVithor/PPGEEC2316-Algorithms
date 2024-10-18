use crate::raw_vec::RawVecError;

pub mod double_hashing;
pub mod linear_probing;

#[derive(Debug)]
pub enum HashTableError {
    Memory(RawVecError),
    Full,
    Empty,
}
