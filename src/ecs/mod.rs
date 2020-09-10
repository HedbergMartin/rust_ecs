//mod sparse_set;


//TODO all sub mods private?
pub mod sparse_set;

#[macro_use]
pub mod systems;

#[macro_use]
pub mod cm;

pub use cm::Component;

mod entity_handler;

pub use entity_handler::Entity;

use std::cell::Ref;
use std::cell::RefMut;
use std::cell::RefCell;

// use std::sync::RwLock;

pub struct Manager {
    ent_handler: RefCell<entity_handler::EntityHandler>,
    schedule: RefCell<std::collections::HashMap<String, systems::System>>,
    comp_manager: RefCell<cm::ComponentManager>,
}

pub type ComponentView<'l> = Ref<'l, cm::ComponentManager>;

impl Manager {
    pub fn new() -> Self {
        Manager {
            ent_handler: RefCell::new(entity_handler::EntityHandler::new()),
            schedule: RefCell::new(std::collections::HashMap::new()),
            comp_manager: RefCell::new(cm::ComponentManager::new()),
        }
    }

    pub fn add_entity(&self) -> Entity {
        self.ent_handler.borrow_mut().new_entity()
    }

	pub fn kill_entity(&self, entity: Entity) {
        self.ent_handler.borrow_mut().kill_entity(entity);
        self.comp_manager.borrow().clean_components(entity);
    }

    pub fn entity_alive(&self, entity: Entity) -> bool {
        self.ent_handler.borrow().is_alive(entity)
    }

    pub fn add_component<T: Component >(&self, entity: Entity, component: T) {
        self.comp_manager.borrow_mut().add_component(entity, component);
    }

    fn get_comp_manager(&self) -> Ref<'_, cm::ComponentManager> {
        self.comp_manager.borrow()
    }

    pub fn get_comp_manager_mut(&self) -> RefMut<'_, cm::ComponentManager> {
        self.comp_manager.borrow_mut()
    }

    pub fn register_task<F: 'static + Fn(ComponentView)>(&self, name: &str, func: F) {
        self.schedule.borrow_mut().insert(String::from(name), systems::System::new(func));
    }

    pub fn run_task(&self, name: &str) {
        let n = String::from(name);
        match self.schedule.borrow().get(&n) {
            Some(task) => task.run(self.comp_manager.borrow()),
            None => print!("No task found with ID {}\n", name),
        }
    }

    pub fn print_components<T: Component>(&self) {
        match self.get_comp_manager().get_components::<T>() {
            Some(comp) => comp.print(),
            None => print!("No comp of type {:?} found", std::any::type_name::<T>()),
        }
    }
}

#[macro_export]
macro_rules! entity_with {
    ($m:expr, $($comp:expr),*) => {{
        let e = crate::ecs::Manager::add_entity($m);
        $(
            crate::ecs::Manager::add_component($m, e, $comp);
        )*
        e
    }};
}