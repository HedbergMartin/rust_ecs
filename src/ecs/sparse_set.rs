use crate::Entity;

type SparseMap = std::collections::HashMap<Entity, usize>;

///
/// A sparse set data type. Made to have very efficient insert and iterations.
/// This sparse set is specially made for this entity component system and
/// will therefore only work with entitys as key.
/// 
pub struct SparseSet<T> {
    comp_array: Vec<T>,
    entity_array: Vec<Entity>,
    sparse_array: SparseMap,
    next_group: usize,
}

impl<T> SparseSet<T> {

    ///
    /// Creates a new sparse set. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let set = SparseSet::new::<i32>();
    /// ```
    pub fn new() -> Self {
        SparseSet {
            comp_array: Vec::<T>::new(),
            entity_array: Vec::<Entity>::new(),
            sparse_array: SparseMap::new(),
            next_group: 0,
        }
    }

    ///
    /// Adds a new entry to the sparse set. If entry allready exsists, nothing happens.
    /// This might change later though.
    ///
    /// # Examples
    /// 
    /// "entity" is an Entity, see rust_ecs::Manager for more detail.
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::new::<i32>();
    /// 
    /// set.add(entity, 2);
    /// ```
    pub fn add(&mut self, entity: Entity, value: T) {
        
        //std::mem::replace(self.comp_array.get_mut(*index).unwrap(), value);
        //TODO check for existing value at index
        if !self.sparse_array.contains_key(&entity) {
            self.sparse_array.insert(entity, self.comp_array.len());
            self.comp_array.push(value);
            self.entity_array.push(entity);
        }
    }

    ///
    /// Returns an option of type &T belonging to the given entity. 
    ///
    /// # Examples
    /// 
    /// "entity" is an Entity, see rust_ecs::Manager for more detail.
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::new::<i32>();
    /// 
    /// set.add(entity, 2);
    /// 
    /// assert_eq!(2, *set.get(entity).unwrap());
    /// ```
    pub fn get(&self, entity: &Entity) -> Option<&T> {
        match self.sparse_array.get(entity) {
            Some(i) => self.comp_array.get(*i),
            None => None,
        }
    }

    ///
    /// Returns an option of type &T containing the T at the given position in the packed array.
    ///
    /// # Examples
    /// 
    /// "entity" is an Entity, see rust_ecs::Manager for more detail.
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::new::<i32>();
    /// 
    /// set.add(entity, 2);
    /// 
    /// assert_eq!(2, *set.component_at(0).unwrap());
    /// ```
    pub fn component_at(&self, index: usize) -> Option<&T> {
        self.comp_array.get(index)
    }

    ///
    /// Returns an option of type &mut T containing the mutable T at the given position in the packed array.
    ///
    /// # Examples
    /// 
    /// "entity" is an Entity, see rust_ecs::Manager for more detail.
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::new::<i32>();
    /// 
    /// set.add(entity, 2);
    /// 
    /// assert_eq!(2, *set.component_at_mut(0).unwrap());
    /// ```
    pub fn component_at_mut(&mut self, index: usize) -> Option<&mut T> {
        self.comp_array.get_mut(index)
    }

    ///
    /// Returns an option of type &Entity containing the entity which owns the i:th component in the packed array.
    ///
    /// # Examples
    /// 
    /// "entity" is an Entity, see rust_ecs::Manager for more detail.
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::new::<i32>();
    /// 
    /// set.add(entity, 2);
    /// 
    /// assert_eq!(&entity, set.entity_at(0).unwrap());
    /// ```
    pub fn entity_at(&self, index: usize) -> Option<&Entity> {
        self.entity_array.get(index)
    }

    ///
    /// Returns whether or not a certain entity is in the sparse set.
    /// Aka if the entity has the component T.
    ///
    /// # Examples
    /// 
    /// "entity" is an Entity, see rust_ecs::Manager for more detail.
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::new::<i32>();
    /// 
    /// assert!(!set.contains(entity));
    /// ```
    pub fn contains(&self, entity: &Entity) -> bool {
        match self.sparse_array.get(entity) {
            Some(_) => true,
            None => false,
        }
    }

    ///
    /// Returns the amount of components/entities in the sparse set.
    ///
    /// # Examples
    /// 
    /// "entity" is an Entity, see rust_ecs::Manager for more detail.
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::new::<i32>();
    /// 
    /// set.add(entity1, 2);
    /// set.add(entity2, 2);
    /// set.add(entity3, 2);
    /// 
    /// assert_eq!(3, set.len());
    /// ```
    pub fn len(&self) -> usize {
        self.entity_array.len()
    }

    pub fn print(&self) {
        print!("Entitys ");
        for i in 0..self.len() {
            print!("{:?} ({}), ", self.entity_array.get(i).unwrap(), i);
        }
        print!("\n");
        //TODO print sparse_array
    }

    ///
    /// Groups a given entity. Aka moves it to the end of the current group.
    /// Should not be used outside of the rust_ecs crate. Needs to be public because
    /// of grouping macro.
    pub fn group(&mut self, entity: &Entity) {
        if self.sparse_array.contains_key(entity) {
            //Should never panic
            let entity_array_index = *self.sparse_array.get(entity).unwrap();
            if self.next_group < entity_array_index {
                let ungrouped = *self.entity_array.get(self.next_group).unwrap();
                let temp = self.sparse_array.insert(*entity, self.next_group).unwrap();
                self.sparse_array.insert(ungrouped, temp);
                self.comp_array.swap(self.next_group, entity_array_index);
                self.entity_array.swap(self.next_group, entity_array_index);
                self.next_group += 1;
            } else if self.next_group == entity_array_index {
                self.next_group += 1;
            }
        } else {
            print!("No entity with id {:?}", entity);
        }
    }

    ///
    /// Groups a given entity. Aka moves it to the end of the current group.
    /// Should not be used outside of the rust_ecs crate. Needs to be public because
    /// of grouping macro.
    pub fn ungroup(&mut self, entity: &Entity) {
        if self.sparse_array.contains_key(entity) {
            //Should never panic
            let entity_array_index = *self.sparse_array.get(entity).unwrap();
            if entity_array_index < self.next_group - 1 {
                let last_grouped = *self.entity_array.get(self.next_group-1).unwrap();
                let temp = self.sparse_array.insert(*entity, self.next_group-1).unwrap();
                self.sparse_array.insert(last_grouped, temp);
                self.comp_array.swap(self.next_group-1, entity_array_index);
                self.entity_array.swap(self.next_group-1, entity_array_index);
                self.next_group -= 1;
            } else if entity_array_index == self.next_group - 1 {
                self.next_group -= 1;   
            }
        } else {
            print!("No entity with id {:?}", entity);
        }
    }

    pub(crate) fn get_group_size(&self) -> usize {
        self.next_group
    }

    ///
    /// Removes an entity and its component from the set. Ungroups if they are grouped.
    /// Removes are done with swap to stay somewhat efficient.
    ///
    /// # Examples
    /// 
    /// "entity" is an Entity, see rust_ecs::Manager for more detail.
    /// 
    /// ```
    /// use rust_ecs::sparse_set::SparseSet;
    /// 
    /// let mut set = SparseSet::new::<i32>();
    /// 
    /// set.add(entity, 2);
    /// set.remove(entity);
    /// 
    /// assert_eq!(0, set.len());
    /// ```
    pub fn remove(&mut self, entity: &Entity) {
        if self.sparse_array.contains_key(entity) {
            let index = *self.sparse_array.get(&entity).unwrap();
            if index < self.next_group {
                //TODO ungroup by index for efficency
                self.ungroup(&entity);
            }
            self.sparse_array.remove(entity);
            self.comp_array.swap_remove(index);
            self.entity_array.swap_remove(index);

            //Updates sparse array if a swap occured
            if index < self.len() {
                let swaped_entity = self.entity_array.get(index).unwrap();
                self.sparse_array.insert(*swaped_entity, index);
            }
        }
    }
}