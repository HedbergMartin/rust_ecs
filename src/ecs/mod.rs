//mod sparse_set;

pub mod sparse_set;
mod cm;
#[macro_use]
pub mod system;

pub type Entity = usize;

use std::cell::Ref;
use std::cell::RefMut;
use std::cell::RefCell;

pub struct Manager {
    entities: Vec<i32>,
    comp_manager: RefCell<cm::ComponentManager>,
}

pub type ComponentView<'l> = Ref<'l, cm::ComponentManager>;

impl Manager {
    pub fn new() -> Self {
        Manager {
            entities: Vec::new(),
            comp_manager: RefCell::new(cm::ComponentManager::new()),
        }
    }

    pub fn add_component<T: std::any::Any >(&self, entity: &Entity, component: T) {
        self.comp_manager.borrow_mut().add_component(entity, component);
    }

    pub fn get_comp_manager(&self) -> Ref<'_, cm::ComponentManager> {
        self.comp_manager.borrow()
    }

    pub fn get_comp_manager_mut(&self) -> RefMut<'_, cm::ComponentManager> {
        self.comp_manager.borrow_mut()
    }

    pub fn run<F>(&self, func: F)
    where F: FnOnce(ComponentView) {
        func(self.get_comp_manager());
    }
}