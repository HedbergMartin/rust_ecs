
use std::cell::Cell;
type SparseMap = std::collections::HashMap<usize, usize>;

pub struct SparseSet<T> {
    pub dense_array: Vec<Cell<T>>,
    sparse_array: SparseMap,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        SparseSet {
            dense_array: Vec::<Cell<T>>::new(),
            sparse_array: SparseMap::new(),
        }
    }

    pub fn add(&mut self, index: &usize, value: T) {
        //TODO check for existing value at index
        self.sparse_array.insert(*index, self.dense_array.len());
        self.dense_array.push(Cell::new(value));
    }

    pub fn get(&self, index: &usize) -> Option<T>
    where T: std::marker::Copy {
        match self.sparse_array.get(index) {
            Some(i) => Some(self.dense_array.get(*i).unwrap().get()), //TODO maybe inefficent?
            None => None,
        }
    }

    pub fn get_dense_index(&self, index: &usize) -> Option<T>
    where T: std::marker::Copy {
        match self.dense_array.get(*index) {
            Some(c) => Some(c.get()), //TODO maybe inefficent?
            None => None,
        }
    }

    /*pub fn get_mut(&mut self, index: &usize) -> Option<&mut T> {
        match self.sparse_array.get(index) {
            Some(i) => self.dense_array.get_mut(*i),
            None => None,
        }
    }*/

    pub fn len(&self) -> usize {
        self.dense_array.len()
    }
}