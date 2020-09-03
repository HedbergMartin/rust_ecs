use crate::ecs::Entity;

type SparseMap = std::collections::HashMap<Entity, usize>;

pub struct SparseSet<T> {
    comp_array: Vec<T>,
    entity_array: Vec<Entity>,
    sparse_array: SparseMap,
    group: usize,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        SparseSet {
            comp_array: Vec::<T>::new(),
            entity_array: Vec::<Entity>::new(),
            sparse_array: SparseMap::new(),
            group: 0,
        }
    }

    pub fn add(&mut self, entity: Entity, value: T) {
        //TODO check for existing value at index
        self.sparse_array.insert(entity, self.comp_array.len());
        self.comp_array.push(value);
        self.entity_array.push(entity);
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
            print!("{} ({}), ", self.entity_array.get(i).unwrap(), i);
        }
        print!("\n");
        //TODO print sparse_array
    }

    pub fn swap(&mut self, entity: &Entity) {
        match self.sparse_array.get(entity) {
            Some(_) => {
                //Should never panic
                let entity_array_index = *self.sparse_array.get(entity).unwrap();
                let ungrouped = *self.entity_array.get(self.group).unwrap();
                let temp = self.sparse_array.insert(*entity, self.group).unwrap();
                self.sparse_array.insert(ungrouped, temp);
                self.comp_array.swap(self.group, entity_array_index);
                self.entity_array.swap(self.group, entity_array_index);
                self.group += 1;
            },
            None => print!("No entity with id {}", entity),
        }
    }

    pub fn get_group_size(&self) -> usize {
        self.group
    }
}