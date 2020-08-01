
type SparseMap = std::collections::HashMap<usize, usize>;

pub struct SparseSet<T> {
    dense_array: Vec<T>,
    sparse_array: SparseMap,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        SparseSet {
            dense_array: Vec::<T>::new(),
            sparse_array: SparseMap::new(),
        }
    }

    pub fn add(&mut self, index: &usize, value: T) {
        //TODO check for existing value at index
        self.sparse_array.insert(*index, self.dense_array.len());
        self.dense_array.push(value);
    }

    pub fn get(&self, index: &usize) -> Option<&T> {
        match self.sparse_array.get(index) {
            Some(i) => self.dense_array.get(*i),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.dense_array.len()
    }
}