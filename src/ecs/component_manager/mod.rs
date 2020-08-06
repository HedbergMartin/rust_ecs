mod family_manager;

use std::cell::RefCell;
use crate::ecs::sparse_set;
use crate::ecs::Entity;

pub struct ComponentManager {
    family_container: family_manager::Container
}

impl ComponentManager {
    pub fn new() -> Self {
        ComponentManager { family_container: family_manager::Container::new() }
    }

    pub fn add_component<T: std::any::Any >(&mut self, entity: &Entity, component: T) {
        //print!("Adding comp... ");
        //TODO Redo with add inside of family manager?
        match self.family_container.get_mut::<T>() {
            Some(family) => {
                //print!("Found family and adding!\n");
                family.components.borrow_mut().add(entity, component);
            },
            None => {
                //print!("Creating family... ");
                self.family_container.add::<T>(family_manager::Family::new());
                self.add_component::<T>(entity, component);
                return;
            },
        }
    }
    
    pub fn get_components<T: std::any::Any>(&self) -> Option<std::cell::Ref<'_, sparse_set::SparseSet<T>>> {
        //print!("Getting components of type {}!\n", std::any::type_name::<T>());
        match self.family_container.get::<T>() {
            Some(family) =>  Some(family.components.borrow()),
            None => None,
        }
    }

    pub fn get_components_mut<T: std::any::Any>(&self) -> Option<std::cell::RefMut<'_, sparse_set::SparseSet<T>>> {
        //print!("Getting components of type {}!\n", std::any::type_name::<T>());
        match self.family_container.get::<T>() {
            Some(family) =>  Some(family.components.borrow_mut()),
            None => None,
        }
    }

    
    /*pub fn get_component<T: std::any::Any>(&self, entity: &Entity) -> Option<T> 
    where T: std::marker::Copy {
        //print!("Getting component of type {} for entity with ID {}!\n", std::any::type_name::<T>(), entity);
        match self.family_container.borrow().get::<T>() {
            Some(family) =>  family.components.borrow().get(entity),
            None => None,
        }
    }*/
}