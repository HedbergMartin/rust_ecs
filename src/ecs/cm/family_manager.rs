use crate::sparse_set::*;
use std::cell::RefCell;
use crate::Entity;

pub struct Family<T> {
    pub components: RefCell<SparseSet<Entity, T>>,
}

impl<T> Family<T> {
    pub fn new() -> Self {
        Family{ components: RefCell::new(SparseSet::new()), }
    }
}

pub struct Container {
    families: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>,
}

impl Container {
    pub fn new() -> Self {
        Container {
            families: std::collections::HashMap::new(),
        }
    }

    pub fn add_family<T: std::any::Any>(&mut self, family: Family<T>) {
        self.families.insert(std::any::TypeId::of::<T>(), Box::new(family));
    }

    pub fn get_family<T: std::any::Any>(& self) -> Option<& Family<T>> {
        if let Some(b) = self.families.get(&(std::any::TypeId::of::<T>())) {
            return b.downcast_ref::<Family<T>>();
        }
        None
    }

    pub fn get_family_mut<T: std::any::Any>(&mut self) -> Option<&mut Family<T>> {
        match self.families.get_mut(&(std::any::TypeId::of::<T>())) {
            Some(b) => b.downcast_mut::<Family<T>>(),
            None => None,
        }
    }
}