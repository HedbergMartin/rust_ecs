//mod sparse_set;


//TODO all sub mods private?
pub mod sparse_set;

#[macro_use]
pub mod systems;

#[macro_use]
pub mod cm;

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

    pub fn add_component<T: Groupable >(&self, entity: Entity, component: T) {
        self.comp_manager.borrow_mut().add_component(entity, component);
    }

    fn get_comp_manager(&self) -> Ref<'_, cm::ComponentManager> {
        self.comp_manager.borrow()
    }

    pub fn get_comp_manager_mut(&self) -> RefMut<'_, cm::ComponentManager> {
        self.comp_manager.borrow_mut()
    }

    pub fn register_task<F: 'static + Fn(ComponentView)>(&self, name: &str, func: F) {
        self.schedule.borrow_mut().insert(String::from(name), systems::System::new(name, func));
    }

    pub fn run_task(&self, name: &str) {
        let n = String::from(name);
        match self.schedule.borrow().get(&n) {
            Some(task) => task.run(self.comp_manager.borrow()),
            None => print!("No task found with ID {}\n", name),
        }
    }

    pub fn print_components<T: Groupable>(&self) {
        match self.get_comp_manager().get_components::<T>() {
            Some(comp) => comp.print(),
            None => print!("No comp of type {:?} found", std::any::type_name::<T>()),
        }
    }
}

pub trait Groupable: 'static {
    fn sort(cm: &cm::ComponentManager, entity: &Entity);
}

#[macro_export]
macro_rules! entity {
    ($m:expr, $($comp:expr),*) => {{
        let e = crate::ecs::Manager::add_entity($m);
        $(
            crate::ecs::Manager::add_component($m, e, $comp);
        )*
        e
    }};
}