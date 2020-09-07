use crate::ecs::Entity;

type SparseMap = std::collections::HashMap<Entity, usize>;

pub struct SparseSet<T> {
    comp_array: Vec<T>,
    entity_array: Vec<Entity>,
    sparse_array: SparseMap,
    next_group: usize,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        SparseSet {
            comp_array: Vec::<T>::new(),
            entity_array: Vec::<Entity>::new(),
            sparse_array: SparseMap::new(),
            next_group: 0,
        }
    }

    pub fn add(&mut self, entity: Entity, value: T) {
        
        //std::mem::replace(self.comp_array.get_mut(*index).unwrap(), value);
        //TODO check for existing value at index
        if !self.sparse_array.contains_key(&entity) {
            self.sparse_array.insert(entity, self.comp_array.len());
            self.comp_array.push(value);
            self.entity_array.push(entity);
        }
    }

    pub fn get(&self, entity: &Entity) -> Option<&T> {
        match self.sparse_array.get(entity) {
            Some(i) => self.comp_array.get(*i),
            None => None,
        }
    }

    pub fn component_at(&self, index: usize) -> Option<&T> {
        self.comp_array.get(index)
    }

    pub fn component_at_mut(&mut self, index: usize) -> Option<&mut T> {
        self.comp_array.get_mut(index)
    }

    pub fn entity_at(&self, index: usize) -> Option<&Entity> {
        self.entity_array.get(index)
    }

    pub fn contains(&self, entity: &Entity) -> bool {
        match self.sparse_array.get(entity) {
            Some(_) => true,
            None => false,
        }
    }

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

    // TODO Redo group and ungroup functions
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

    pub fn get_group_size(&self) -> usize {
        self.next_group
    }

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