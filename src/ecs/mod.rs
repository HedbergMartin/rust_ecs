//mod sparse_set;

pub mod sparse_set;
pub mod systems;
mod cm;

pub type Entity = usize;

use std::cell::Ref;
use std::cell::RefMut;
use std::cell::RefCell;

pub struct Manager {
    entities: Vec<i32>,
    schedule: RefCell<Vec<systems::System>>,
    comp_manager: RefCell<cm::ComponentManager>,
}

pub type ComponentView<'l> = Ref<'l, cm::ComponentManager>;

impl Manager {
    pub fn new() -> Self {
        Manager {
            entities: Vec::new(),
            schedule: RefCell::new(Vec::new()),
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

    pub fn register_task<F: 'static + Fn(ComponentView)>(&self, func: F) {
        self.schedule.borrow_mut().push(systems::System::new(func));
    }

    pub fn run_task(&self, index: usize) {
        match self.schedule.borrow().get(index) {
            Some(task) => task.run(self.comp_manager.borrow()),
            None => print!("No task found with ID {}\n", index),
        }
    }
}