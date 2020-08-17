//mod sparse_set;

pub mod sparse_set;
pub mod systems;
mod cm;

pub type Entity = usize;

use std::cell::Ref;
use std::cell::RefMut;
use std::cell::RefCell;

use std::sync::RwLock;

pub struct Manager {
    entities: Vec<i32>,
    schedule: RefCell<std::collections::HashMap<String, systems::System>>,
    comp_manager: RefCell<cm::ComponentManager>,
    next_entity_id: RwLock<Entity>,
}

pub type ComponentView<'l> = Ref<'l, cm::ComponentManager>;

impl Manager {
    pub fn new() -> Self {
        Manager {
            entities: Vec::new(),
            schedule: RefCell::new(std::collections::HashMap::new()),
            comp_manager: RefCell::new(cm::ComponentManager::new()),
            next_entity_id: RwLock::new(0),
        }
    }

    pub fn add_entity(&self) -> Entity {
        let e = *self.next_entity_id.read().unwrap();
        *self.next_entity_id.write().unwrap() = e+1;

        return e;
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

    pub fn register_task<'name, F: 'static + Fn(ComponentView)>(&self, name: &str, func: F) {
        self.schedule.borrow_mut().insert(String::from(name), systems::System::new(name, func));
    }

    pub fn run_task(&self, name: &str) {
        let n = String::from(name);
        match self.schedule.borrow().get(&n) {
            Some(task) => task.run(self.comp_manager.borrow()),
            None => print!("No task found with ID {}\n", name),
        }
    }
}