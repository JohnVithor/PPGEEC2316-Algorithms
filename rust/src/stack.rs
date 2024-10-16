extern crate alloc;
use alloc::alloc::{alloc, dealloc};
use core::{alloc::Layout, mem, ptr::NonNull};

#[derive(Debug)]
pub enum StackError {
    OutOfMemory,
    Full,
    Empty,
}

pub struct Stack<T> {
    buffer: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T: core::fmt::Debug> core::fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Stack")
            .field("buffer", unsafe {
                &core::slice::from_raw_parts(self.buffer.as_ptr(), self.len)
            })
            .field("cap", &self.cap)
            .field("len", &self.len)
            .finish()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            buffer: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }
}

impl<T> Stack<T> {
    pub fn new(cap: usize) -> Result<Self, StackError> {
        let layout = match Layout::array::<T>(cap) {
            Ok(layout) => layout,
            Err(_) => return Err(StackError::OutOfMemory),
        };

        if layout.size() >= isize::MAX as usize {
            return Err(StackError::OutOfMemory);
        }

        let ptr = unsafe { alloc(layout) };

        let ptr = match NonNull::new(ptr as *mut T) {
            Some(p) => p,
            None => return Err(StackError::OutOfMemory),
        };
        Ok(Self {
            buffer: ptr,
            len: 0,
            cap,
        })
    }

    pub fn push(&mut self, value: T) -> Result<(), StackError> {
        if self.len == self.cap {
            return Err(StackError::Full);
        }

        unsafe {
            self.buffer.as_ptr().add(self.len).write(value);
        }
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        if self.len == 0 {
            return Err(StackError::Empty);
        }

        self.len -= 1;
        unsafe { Ok(self.buffer.as_ptr().add(self.len).read()) }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        if self.cap != 0 && mem::size_of::<T>() != 0 {
            while self.pop().is_ok() {}
            unsafe {
                dealloc(
                    self.buffer.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}
