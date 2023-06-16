use std::{marker::PhantomData, ops::Index};

pub struct Arena<T> {
    entries: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn allocate(&mut self, v: T) -> Idx<T> {
        let index = self.current_index();
        self.entries.push(v);
        Idx::new(index as u32)
    }

    fn current_index(&self) -> usize {
        self.entries.len()
    }
}

impl<T> Index<Idx<T>> for Arena<T> {
    type Output = T;

    fn index(&self, index: Idx<T>) -> &Self::Output {
        &self.entries[index.value as usize]
    }
}

pub struct Idx<T> {
    value: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Idx<T> {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

// NOTE: We manually implement instances such that `T` doesn't require
// `Copy` and `Clone`, unlike if we did it through the `derive` macro.

impl<T> Copy for Idx<T> {}

impl<T> Clone for Idx<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> std::fmt::Debug for Idx<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Idx").field("value", &self.value).finish()
    }
}
