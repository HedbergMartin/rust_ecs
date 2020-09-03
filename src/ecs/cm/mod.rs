mod family_manager;

use crate::ecs::sparse_set;
use crate::ecs::Entity;
use crate::ecs::Groupable;

pub type View<'l, T> = std::cell::Ref<'l, sparse_set::SparseSet<T>>;
pub type ViewMut<'l, T> = std::cell::RefMut<'l, sparse_set::SparseSet<T>>;

pub struct ComponentManager {
    family_container: family_manager::Container
}

impl ComponentManager {
    pub fn new() -> Self {
        ComponentManager { family_container: family_manager::Container::new() }
    }

    pub fn add_component<T: Groupable >(&mut self, entity: Entity, component: T) {
        match self.family_container.get_family_mut::<T>() {
            Some(family) => {
                family.components.borrow_mut().add(entity, component);
                T::sort(&self, &entity);
            },
            None => {
                self.family_container.add_family::<T>(family_manager::Family::new());
                self.add_component::<T>(entity, component);
                return;
            },
        }
    }
    
    pub fn get_components<T: Groupable>(&self) -> Option<View<T>> {
        match self.family_container.get_family::<T>() {
            Some(family) =>  {
                Some(family.components.borrow())
            },
            None => None,
        }
    }
    
    pub fn get_components_mut<T: Groupable>(&self) -> Option<ViewMut<T>> {
        match self.family_container.get_family::<T>() {
            Some(family) =>  {
                Some(family.components.borrow_mut())
            },
            None => None,
        }
    }

    pub fn contains<T: Groupable>(&self, entity: &Entity) -> bool {
        match self.get_components::<T>() {
            Some(c) => c.contains(entity),
            None => false,
        }
    }
}

#[macro_export]
macro_rules! group {
    ($head:ty) => {
        impl ecs::Groupable for $head {
            fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::Entity) {
                
            }
        }
    };
    ($head:ty, $($tail:ty),+) => {
        group_raw!($head, $($tail),+;);
    };
}

// Ugly af but works
#[macro_export]
macro_rules! group_raw {
    ($head:ty, $($queue:ty),+; $($done:ty),+) => {
        impl ecs::Groupable for $head {
            fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::Entity) {
                //TODO unwrap?
                // print!("Sorting hps\n");
                //TODO not mut?
                if $(cm.contains::<$queue>(entity))&&+ && $(cm.contains::<$done>(entity))&&+ {
                    cm.get_components_mut::<$head>().unwrap().group(entity);
                    $(
                    cm.get_components_mut::<$queue>().unwrap().group(entity);
                    )+
                    $(
                    cm.get_components_mut::<$done>().unwrap().group(entity);
                    )+
                }
            }
        }
        group_raw!($($queue),+; $($done),+, $head);
    };

    ($head:ty; $($done:ty),+) => {
        impl ecs::Groupable for $head {
            fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::Entity) {
                //TODO unwrap?
                // print!("Sorting hps\n");
                //TODO not mut?
                if $(cm.contains::<$done>(entity))&&+ {
                    cm.get_components_mut::<$head>().unwrap().group(entity);
                    $(
                    cm.get_components_mut::<$done>().unwrap().group(entity);
                    )+
                }
            }
        }
    };
    
    ($head:ty, $($queue:ty),+; ) => {
        impl ecs::Groupable for $head {
            fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::Entity) {
                //TODO unwrap?
                // print!("Sorting hps\n");
                //TODO not mut?
                if $(cm.contains::<$queue>(entity))&&+ {
                    cm.get_components_mut::<$head>().unwrap().group(entity);
                    $(
                    cm.get_components_mut::<$queue>().unwrap().group(entity);
                    )+
                }
            }
        }
        group_raw!($($queue),+; $head);
    };
}

// pub struct View<'l, T: std::any::Any> {
//     set_ref: std::cell::Ref<'l, sparse_set::SparseSet<T>>,
// }

// impl<T: std::any::Any> std::ops::Deref for View<'_, T> {
//     type Target = sparse_set::SparseSet<T>;

//     fn deref(&self) -> &Self::Target {
//         &self.set_ref
//     }
// }

// pub struct ViewMut<'l, T: std::any::Any> {
//     set_ref: std::cell::RefMut<'l, sparse_set::SparseSet<T>>,
// }

// impl<'l, T: std::any::Any> std::ops::Deref for ViewMut<'l, T> {
//     type Target = sparse_set::SparseSet<T>;

//     fn deref(&self) -> &Self::Target {
//         &self.set_ref
//     }
// }

// impl<'l, T: std::any::Any> std::ops::DerefMut for ViewMut<'l, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.set_ref
//     }
// }