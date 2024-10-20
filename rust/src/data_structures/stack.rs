use super::raw_vec::{RawVec, RawVecError};

#[derive(Debug)]
pub enum StackError {
    Memory(RawVecError),
    Full,
    Empty,
}

#[derive(Default)]
pub struct Stack<T> {
    buffer: RawVec<T>,
    len: usize,
}

impl<T: core::fmt::Debug> core::fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Stack")
            .field("buffer", unsafe {
                &core::slice::from_raw_parts(self.buffer.data.as_ptr(), self.len).iter()
            })
            .field("cap", &self.buffer.cap)
            .field("len", &self.len)
            .finish()
    }
}

impl<T> Stack<T> {
    pub fn new(cap: usize) -> Result<Self, StackError> {
        let buffer = match RawVec::new(cap) {
            Ok(buffer) => buffer,
            Err(e) => return Err(StackError::Memory(e)),
        };
        Ok(Self { buffer, len: 0 })
    }

    pub fn push(&mut self, value: T) -> Result<(), StackError> {
        if self.len == self.buffer.cap {
            return Err(StackError::Full);
        }

        unsafe {
            self.buffer.data.as_ptr().add(self.len).write(value);
        }
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        if self.len == 0 {
            return Err(StackError::Empty);
        }
        self.len -= 1;
        unsafe { Ok(self.buffer.data.as_ptr().add(self.len).read()) }
    }
}
