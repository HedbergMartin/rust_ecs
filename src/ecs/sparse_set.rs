
use std::collections::HashMap;

///
/// A sparse set data type. Made to have very efficient insert and iterations.
/// This sparse set is specially made for this key component system so it
/// contains speciall features such as grouping.
/// 
pub struct SparseSet<Key, Value> {
    comp_array: Vec<Value>,
    entity_array: Vec<Key>,
    sparse_array: HashMap<Key, usize>,
    next_group: usize,
}

impl<Key, Value> SparseSet<Key, Value>
where Key: std::cmp::Eq + std::hash::Hash + std::fmt::Debug + std::marker::Copy {

    ///
    /// Creates a new sparse set. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let set = SparseSet::<u32, i32>::new();
    /// ```
    pub fn new() -> Self {
        SparseSet {
            comp_array: Vec::new(),
            entity_array: Vec::new(),
            sparse_array: HashMap::new(),
            next_group: 0,
        }
    }

    ///
    /// Adds a new entry to the sparse set. If entry allready exsists, nothing happens.
    /// This might change later though.
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::<u32, i32>::new();
    /// 
    /// set.add(&0, 2);
    /// ```
    pub fn add(&mut self, key: &Key, value: Value) {
        
        //std::mem::replace(self.comp_array.get_mut(*index).unwrap(), value);
        //TODO check for existing value at index
        if !self.sparse_array.contains_key(key) {
            self.sparse_array.insert(*key, self.comp_array.len());
            self.comp_array.push(value);
            self.entity_array.push(*key);
        }
    }

    ///
    /// Returns an option of type &Value belonging to the given key. 
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::<u32, i32>::new();
    /// 
    /// set.add(&0, 2);
    /// 
    /// assert_eq!(2, *set.get(&0).unwrap());
    /// ```
    pub fn get(&self, key: &Key) -> Option<&Value> {
        match self.sparse_array.get(key) {
            Some(i) => self.comp_array.get(*i),
            None => None,
        }
    }

    ///
    /// Returns an option of type &Value containing the Value at the given position in the packed array.
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::<u32, i32>::new();
    /// 
    /// set.add(&21, 2);
    /// 
    /// assert_eq!(2, *set.component_at(0).unwrap());
    /// ```
    pub fn component_at(&self, index: usize) -> Option<&Value> {
        self.comp_array.get(index)
    }

    ///
    /// Returns an option of type &mut Value containing the mutable Value at the given position in the packed array.
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::<u32, i32>::new();
    /// 
    /// set.add(&21, 2);
    /// 
    /// assert_eq!(2, *set.component_at_mut(0).unwrap());
    /// ```
    pub fn component_at_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.comp_array.get_mut(index)
    }

    ///
    /// Returns an option of type &Key containing the key which owns the i:th component in the packed array.
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::<u32, i32>::new();
    /// 
    /// set.add(&21, 2);
    /// 
    /// assert_eq!(21, *set.key_at(0).unwrap());
    /// ```
    pub fn key_at(&self, index: usize) -> Option<&Key> {
        self.entity_array.get(index)
    }

    ///
    /// Returns whether or not a certain key is in the sparse set.
    /// Aka if the key has the component Value.
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::<u32, i32>::new();
    /// 
    /// set.add(&2, 2);
    /// 
    /// assert!(!set.contains(&3));
    /// ```
    pub fn contains(&self, key: &Key) -> bool {
        match self.sparse_array.get(key) {
            Some(_) => true,
            None => false,
        }
    }

    ///
    /// Returns the amount of components/entities in the sparse set.
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::<u32, i32>::new();
    /// 
    /// set.add(&0, 2);
    /// set.add(&1, 2);
    /// set.add(&2, 2);
    /// 
    /// assert_eq!(3, set.len());
    /// ```
    pub fn len(&self) -> usize {
        self.entity_array.len()
    }

    pub fn print(&self) {
        print!("Keys ");
        for i in 0..self.len() {
            print!("{:?} ({}), ", self.entity_array.get(i).unwrap(), i);
        }
        print!("\n");
        //TODO print sparse_array
    }

    ///
    /// Groups a given key. Aka moves it to the end of the current group.
    /// Should not be used outside of the rust_ecs crate. Needs to be public because
    /// of grouping macro.
    pub fn group(&mut self, key: &Key) {
        if self.sparse_array.contains_key(key) {
            //Should never panic
            let entity_array_index = *self.sparse_array.get(key).unwrap();
            if self.next_group < entity_array_index {
                let ungrouped = *self.entity_array.get(self.next_group).unwrap();
                let temp = self.sparse_array.insert(*key, self.next_group).unwrap();
                self.sparse_array.insert(ungrouped, temp);
                self.comp_array.swap(self.next_group, entity_array_index);
                self.entity_array.swap(self.next_group, entity_array_index);
                self.next_group += 1;
            } else if self.next_group == entity_array_index {
                self.next_group += 1;
            }
        } else {
            print!("No key with id {:?}", key);
        }
    }

    ///
    /// Groups a given key. Aka moves it to the end of the current group.
    /// Should not be used outside of the rust_ecs crate. Needs to be public because
    /// of grouping macro.
    pub fn ungroup(&mut self, key: &Key) {
        if self.sparse_array.contains_key(key) {
            //Should never panic
            let entity_array_index = *self.sparse_array.get(key).unwrap();
            if entity_array_index < self.next_group - 1 {
                let last_grouped = *self.entity_array.get(self.next_group-1).unwrap();
                let temp = self.sparse_array.insert(*key, self.next_group-1).unwrap();
                self.sparse_array.insert(last_grouped, temp);
                self.comp_array.swap(self.next_group-1, entity_array_index);
                self.entity_array.swap(self.next_group-1, entity_array_index);
                self.next_group -= 1;
            } else if entity_array_index == self.next_group - 1 {
                self.next_group -= 1;   
            }
        } else {
            print!("No key with id {:?}", key);
        }
    }

    ///
    /// Removes an key and its component from the set. Ungroups if they are grouped.
    /// Removes are done with swap to stay somewhat efficient.
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::<u32, i32>::new();
    /// 
    /// set.add(&0, 2);
    /// set.remove(&0);
    /// 
    /// assert!(!set.contains(&0));
    /// ```
    pub fn remove(&mut self, key: &Key) {
        if self.sparse_array.contains_key(key) {
            let index = *self.sparse_array.get(&key).unwrap();
            if index < self.next_group {
                //TODO ungroup by index for efficency
                self.ungroup(&key);
            }
            self.sparse_array.remove(key);
            self.comp_array.swap_remove(index);
            self.entity_array.swap_remove(index);

            //Updates sparse array if a swap occured
            if index < self.len() {
                let swaped_key = self.entity_array.get(index).unwrap();
                self.sparse_array.insert(*swaped_key, index);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Entity;

    struct TestType { 
        data: i8 
    }
    
    #[test]
    fn empty_sparse_set() {
        let set = SparseSet::<Entity, TestType>::new();
        assert_eq!(set.len(), 0);
    }
    
    #[test]
    fn sparse_set_add_1() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 0});
        assert_eq!(set.len(), 1);
    }
    
    #[test]
    fn sparse_set_add_3() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 0});
        set.add(&Entity::new(1, 0), TestType{data: 0});
        set.add(&Entity::new(2, 0), TestType{data: 0});
        assert_eq!(set.len(), 3);
    }
    
    #[test]
    fn sparse_set_add_same() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 0});
        set.add(&Entity::new(0, 0), TestType{data: 0});
        set.add(&Entity::new(0, 0), TestType{data: 0});
        assert_eq!(set.len(), 1);
    }
    
    #[test]
    fn sparse_set_get_by_keyid() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
        set.add(&Entity::new(1, 0), TestType{data: 3});
        set.add(&Entity::new(2, 0), TestType{data: 4});
        assert_eq!(set.get(&Entity::new(1, 0)).unwrap().data, 3);
        assert_eq!(set.get(&Entity::new(2, 0)).unwrap().data, 4);
        assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
    }
    
    #[test]
    fn sparse_set_data_ordered() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
        set.add(&Entity::new(15, 0), TestType{data: 3});
        set.add(&Entity::new(3, 0), TestType{data: 4});
        assert_eq!(set.component_at(0).unwrap().data, 7);
        assert_eq!(set.component_at(1).unwrap().data, 3);
        assert_eq!(set.component_at(2).unwrap().data, 4);
    
        assert_eq!(*set.key_at(0).unwrap(), Entity::new(0, 0));
        assert_eq!(*set.key_at(1).unwrap(), Entity::new(15, 0));
        assert_eq!(*set.key_at(2).unwrap(), Entity::new(3, 0));
    
        assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
        assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
        assert_eq!(set.get(&Entity::new(3, 0)).unwrap().data, 4);
    }
    
    #[test]
    fn sparse_set_group() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
        set.add(&Entity::new(15, 0), TestType{data: 3});
        set.add(&Entity::new(3, 0), TestType{data: 4});
        set.add(&Entity::new(9, 0), TestType{data: 25});
        set.add(&Entity::new(5, 0), TestType{data: 44});
        set.group(&Entity::new(15, 0));
        set.group(&Entity::new(9, 0));
    
        assert_eq!(set.next_group, 2);
    
        assert_eq!(set.component_at(0).unwrap().data, 3);
        assert_eq!(set.component_at(1).unwrap().data, 25);
        assert_eq!(set.component_at(2).unwrap().data, 4);
        assert_eq!(set.component_at(3).unwrap().data, 7);
        assert_eq!(set.component_at(4).unwrap().data, 44);
    
        assert_eq!(*set.key_at(0).unwrap(), Entity::new(15, 0));
        assert_eq!(*set.key_at(1).unwrap(), Entity::new(9, 0));
        assert_eq!(*set.key_at(2).unwrap(), Entity::new(3, 0));
        assert_eq!(*set.key_at(3).unwrap(), Entity::new(0, 0));
        assert_eq!(*set.key_at(4).unwrap(), Entity::new(5, 0));
        
        assert_eq!(set.get(&Entity::new(9, 0)).unwrap().data, 25);
        assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
        assert_eq!(set.get(&Entity::new(3, 0)).unwrap().data, 4);
        assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
        assert_eq!(set.get(&Entity::new(5, 0)).unwrap().data, 44);
    }
    
    #[test]
    fn sparse_set_group_efficien_edge_case() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
        set.add(&Entity::new(15, 0), TestType{data: 3});
        set.group(&Entity::new(0, 0));
    
        assert_eq!(set.next_group, 1);
    
        assert_eq!(set.component_at(0).unwrap().data, 7);
        assert_eq!(set.component_at(1).unwrap().data, 3);
    
        assert_eq!(*set.key_at(0).unwrap(), Entity::new(0, 0));
        assert_eq!(*set.key_at(1).unwrap(), Entity::new(15, 0));
        
        assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
        assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
    }
    
    #[test]
    fn sparse_set_double_group() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
        set.add(&Entity::new(15, 0), TestType{data: 3});
        set.add(&Entity::new(3, 0), TestType{data: 4});
        set.add(&Entity::new(9, 0), TestType{data: 25});
        set.add(&Entity::new(5, 0), TestType{data: 44});
        set.group(&Entity::new(15, 0));
        set.group(&Entity::new(9, 0));
        set.group(&Entity::new(15, 0));
    
        assert_eq!(set.next_group, 2);
    
        assert_eq!(set.component_at(0).unwrap().data, 3);
        assert_eq!(set.component_at(1).unwrap().data, 25);
        assert_eq!(set.component_at(2).unwrap().data, 4);
        assert_eq!(set.component_at(3).unwrap().data, 7);
        assert_eq!(set.component_at(4).unwrap().data, 44);
    
        assert_eq!(*set.key_at(0).unwrap(), Entity::new(15, 0));
        assert_eq!(*set.key_at(1).unwrap(), Entity::new(9, 0));
        assert_eq!(*set.key_at(2).unwrap(), Entity::new(3, 0));
        assert_eq!(*set.key_at(3).unwrap(), Entity::new(0, 0));
        assert_eq!(*set.key_at(4).unwrap(), Entity::new(5, 0));
        // assert_eq!(*set.key_at(0).unwrap(), 15);
        // assert_eq!(*set.key_at(1).unwrap(), 9);
        // assert_eq!(*set.key_at(2).unwrap(), 3);
        // assert_eq!(*set.key_at(3).unwrap(), 0);
        // assert_eq!(*set.key_at(4).unwrap(), 5);
        
        assert_eq!(set.get(&Entity::new(9, 0)).unwrap().data, 25);
        assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
        assert_eq!(set.get(&Entity::new(3, 0)).unwrap().data, 4);
        assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
        assert_eq!(set.get(&Entity::new(5, 0)).unwrap().data, 44);
    }
    
    #[test]
    fn sparse_set_ungroup() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
        set.add(&Entity::new(15, 0), TestType{data: 3});
        set.add(&Entity::new(3, 0), TestType{data: 4});
        set.add(&Entity::new(9, 0), TestType{data: 25});
        set.add(&Entity::new(5, 0), TestType{data: 44});
        set.group(&Entity::new(15, 0));
        set.group(&Entity::new(9, 0));
        set.ungroup(&Entity::new(15, 0));
        assert_eq!(set.component_at(0).unwrap().data, 25);
        assert_eq!(set.component_at(1).unwrap().data, 3);
        assert_eq!(set.component_at(2).unwrap().data, 4);
        assert_eq!(set.component_at(3).unwrap().data, 7);
        assert_eq!(set.component_at(4).unwrap().data, 44);
    
        assert_eq!(*set.key_at(0).unwrap(), Entity::new(9, 0));
        assert_eq!(*set.key_at(1).unwrap(), Entity::new(15, 0));
        assert_eq!(*set.key_at(2).unwrap(), Entity::new(3, 0));
        assert_eq!(*set.key_at(3).unwrap(), Entity::new(0, 0));
        assert_eq!(*set.key_at(4).unwrap(), Entity::new(5, 0));
        
        assert_eq!(set.get(&Entity::new(9, 0)).unwrap().data, 25);
        assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
        assert_eq!(set.get(&Entity::new(3, 0)).unwrap().data, 4);
        assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
        assert_eq!(set.get(&Entity::new(5, 0)).unwrap().data, 44);
    
        assert_eq!(set.next_group, 1);
    }
    
    #[test]
    fn sparse_set_ungroup_efficien_edge_case() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
        set.add(&Entity::new(15, 0), TestType{data: 3});
        set.group(&Entity::new(0, 0));
        set.ungroup(&Entity::new(0, 0));
    
        assert_eq!(set.next_group, 0);
    
        assert_eq!(set.component_at(0).unwrap().data, 7);
        assert_eq!(set.component_at(1).unwrap().data, 3);
    
        assert_eq!(*set.key_at(0).unwrap(), Entity::new(0, 0));
        assert_eq!(*set.key_at(1).unwrap(), Entity::new(15, 0));
        
        assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
        assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
    }
    
    #[test]
    fn sparse_set_double_ungroup() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
        set.add(&Entity::new(15, 0), TestType{data: 3});
        set.add(&Entity::new(3, 0), TestType{data: 4});
        set.add(&Entity::new(9, 0), TestType{data: 25});
        set.add(&Entity::new(5, 0), TestType{data: 44});
        set.group(&Entity::new(15, 0));
        set.group(&Entity::new(9, 0));
        set.ungroup(&Entity::new(15, 0));
        set.ungroup(&Entity::new(15, 0));
        assert_eq!(set.component_at(0).unwrap().data, 25);
        assert_eq!(set.component_at(1).unwrap().data, 3);
        assert_eq!(set.component_at(2).unwrap().data, 4);
        assert_eq!(set.component_at(3).unwrap().data, 7);
        assert_eq!(set.component_at(4).unwrap().data, 44);
    
        assert_eq!(*set.key_at(0).unwrap(), Entity::new(9, 0));
        assert_eq!(*set.key_at(1).unwrap(), Entity::new(15, 0));
        assert_eq!(*set.key_at(2).unwrap(), Entity::new(3, 0));
        assert_eq!(*set.key_at(3).unwrap(), Entity::new(0, 0));
        assert_eq!(*set.key_at(4).unwrap(), Entity::new(5, 0));
        
        assert_eq!(set.get(&Entity::new(9, 0)).unwrap().data, 25);
        assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
        assert_eq!(set.get(&Entity::new(3, 0)).unwrap().data, 4);
        assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
        assert_eq!(set.get(&Entity::new(5, 0)).unwrap().data, 44);
    
        assert_eq!(set.next_group, 1);
    }
    
    #[test]
    fn sparse_set_remove() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
        set.add(&Entity::new(15, 0), TestType{data: 3});
        set.add(&Entity::new(3, 0), TestType{data: 4});
        set.add(&Entity::new(9, 0), TestType{data: 25});
        set.add(&Entity::new(5, 0), TestType{data: 44});
    
        set.remove(&Entity::new(3, 0));
    
        assert_eq!(set.len(), 4);
    
        assert_eq!(set.component_at(0).unwrap().data, 7);
        assert_eq!(set.component_at(1).unwrap().data, 3);
        assert_eq!(set.component_at(2).unwrap().data, 44);
        assert_eq!(set.component_at(3).unwrap().data, 25);
    
        assert_eq!(*set.key_at(0).unwrap(), Entity::new(0, 0));
        assert_eq!(*set.key_at(1).unwrap(), Entity::new(15, 0));
        assert_eq!(*set.key_at(2).unwrap(), Entity::new(5, 0));
        assert_eq!(*set.key_at(3).unwrap(), Entity::new(9, 0));
        
        assert_eq!(set.get(&Entity::new(9, 0)).unwrap().data, 25);
        assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
        //assert_eq!(set.get(&3), None::<&TestType>);
        assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
        assert_eq!(set.get(&Entity::new(5, 0)).unwrap().data, 44);
    }
    
    #[test]
    fn sparse_set_remove_one() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
    
        set.remove(&Entity::new(0, 0));
    
        assert_eq!(set.len(), 0);
    }
    
    #[test]
    fn sparse_set_remove_last() {
        let mut set = SparseSet::<Entity, TestType>::new();
        set.add(&Entity::new(0, 0), TestType{data: 7});
        set.add(&Entity::new(2, 0), TestType{data: 8});
    
        set.remove(&Entity::new(2, 0));
    
        assert_eq!(set.len(), 1);
    
        assert_eq!(set.component_at(0).unwrap().data, 7);
    
        assert_eq!(*set.key_at(0).unwrap(), Entity::new(0, 0));
        
        assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
    }
}