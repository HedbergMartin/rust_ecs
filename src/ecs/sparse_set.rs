pub type Entity = usize;

type SparseMap = std::collections::HashMap<Entity, usize>;

pub trait Groupable {
    fn get_group(&self) -> usize;
    fn get_parsed(&self) -> usize;
    fn group<F: FnMut(Entity) -> bool>(&mut self, swap_callback: F);
    fn swap(&mut self, entity: &Entity);
}


pub struct SparseSet<T> {
    comp_array: Vec<T>,
    entity_array: Vec<Entity>,
    sparse_array: SparseMap,
    group: usize,
    parsed: usize,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        SparseSet {
            comp_array: Vec::<T>::new(),
            entity_array: Vec::<Entity>::new(),
            sparse_array: SparseMap::new(),
            group: 0,
            parsed: 0,
        }
    }

    //TODO should entity be ref?
    pub fn add(&mut self, entity: &Entity, value: T) {
        //TODO check for existing value at index
        self.sparse_array.insert(*entity, self.comp_array.len());
        self.comp_array.push(value);
        self.entity_array.push(*entity);
    }

    pub fn get(&self, entity: &Entity) -> Option<&T> {
        match self.sparse_array.get(entity) {
            Some(i) => self.comp_array.get(*i),
            None => None,
        }
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
}

impl<T> Groupable for SparseSet<T> {
    fn get_group(&self) -> usize {
        self.group
    }

    fn get_parsed(&self) -> usize {
        self.parsed
    }

    fn group<F: FnMut(Entity) -> bool>(&mut self, mut swap_callback: F) {

        //TODO allready grouped?
        while self.parsed < self.len() {
            let current = *self.entity_array.get(self.parsed).unwrap();
            if swap_callback(current) {
                self.swap(&current);
            }
            
            self.parsed += 1;
        }
    }

    fn swap(&mut self, entity: &Entity) {
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
}