use std::{collections::BTreeSet, ops::Index};

/// Immutable String Pool without Interning
#[derive(Default)]
pub struct StringPool {
    pool: Vec<String>,
    free: BTreeSet<usize>,
}

impl StringPool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc(&mut self, s: String) -> usize {
        if self.free.is_empty() {
            self.pool.push(s);
            self.pool.len() - 1
        } else {
            let id = self.free.pop_last().unwrap();
            self.pool[id] = s;
            id
        }
    }

    pub fn get(&self, id: usize) -> Option<&str> {
        if id >= self.pool.len() || self.free.get(&id).is_some() {
            return None;
        }
        Some(&self.pool[id])
    }

    pub fn _free(&mut self, id: usize) {
        debug_assert!(id < self.pool.len());
        self.free.insert(id);
    }
}

impl Index<usize> for StringPool {
    type Output = String;
    fn index(&self, id: usize) -> &Self::Output {
        &self.pool[id]
    }
}