mod family_manager;

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
        match self.family_container.get_family_mut::<T>() {
            Some(family) => {
                //print!("Found family and adding!\n");
                family.components.borrow_mut().add(entity, component);
            },
            None => {
                //print!("Creating family... ");
                self.family_container.add_family::<T>(family_manager::Family::new());
                self.add_component::<T>(entity, component);
                return;
            },
        }
    }
    
    pub fn get_components<T: std::any::Any>(&self) -> Option<View<T>> {
        
        match self.family_container.get_family::<T>() {
            Some(family) =>  {
                Some(View{ set_ref: family.components.borrow() })
            },
            None => None,
        }
    }
    
    pub fn get_components_mut<T: std::any::Any>(&self) -> Option<ViewMut<T>> {
        
        match self.family_container.get_family::<T>() {
            Some(family) =>  {
                Some(ViewMut{ set_ref: family.components.borrow_mut() })
            },
            None => None,
        }
    }
}

pub struct View<'l, T: std::any::Any> {
    set_ref: std::cell::Ref<'l, sparse_set::SparseSet<T>>,
}

impl<T: std::any::Any> std::ops::Deref for View<'_, T> {
    type Target = sparse_set::SparseSet<T>;

    fn deref(&self) -> &sparse_set::SparseSet<T> {
        &self.set_ref
    }
}

pub struct ViewMut<'l, T: std::any::Any> {
    set_ref: std::cell::RefMut<'l, sparse_set::SparseSet<T>>,
}

impl<'l, T: std::any::Any> std::ops::Deref for ViewMut<'l, T> {
    type Target = sparse_set::SparseSet<T>;

    fn deref(&self) -> &sparse_set::SparseSet<T> {
        &self.set_ref
    }
}

impl<'l, T: std::any::Any> std::ops::DerefMut for ViewMut<'l, T> {
    fn deref_mut(&mut self) -> &mut sparse_set::SparseSet<T> {
        &mut self.set_ref
    }
}