//mod sparse_set;

mod family_manager;
mod sparse_set;
pub use family_manager::*;

pub type Entity = usize;

pub struct Manager {
    entities: Vec<i32>,
    container: family_manager::Container,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            entities: Vec::new(),
            container: family_manager::Container::new(),
        }
    }

    pub fn add_component<T : std::any::Any + std::default::Default>(&mut self, entity: &Entity) {
        print!("Adding comp... ");
        let a = self.entities.len();
        match self.container.get_mut::<T>() {
            Some(family) => {
                print!("Found family and adding!\n");
                family.components.add(entity, T::default());
            },
            None => {
                print!("Creating family... ");
                self.container.add(family_manager::Family {test_id: 0, components: sparse_set::SparseSet::<T>::new()});
                self.add_component::<T>(entity);
                return;
            },
        }
    }

    pub fn get_components<T : std::any::Any>(&self) -> Option<& sparse_set::SparseSet<T>> {
        print!("Getting component!\n");
        match self.container.get::<T>() {
            Some(family) =>  Some(&family.components),
            None => None,
        }
    }
}