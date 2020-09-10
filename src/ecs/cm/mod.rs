mod family_manager;

use crate::ecs::sparse_set;
use crate::ecs::Entity;

pub type View<'l, T> = std::cell::Ref<'l, sparse_set::SparseSet<T>>;
pub type ViewMut<'l, T> = std::cell::RefMut<'l, sparse_set::SparseSet<T>>;

pub trait Component: 'static {
    fn group(cm: &ComponentManager, entity: &Entity);
}

pub struct ComponentManager {
    family_container: family_manager::Container,
	cleans: Vec<Box<dyn Fn(&ComponentManager, Entity)>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        ComponentManager {
            family_container: family_manager::Container::new(),
			cleans: Vec::new(),
        }
    }

    pub fn add_component<T: Component >(&mut self, entity: Entity, component: T) {
        match self.family_container.get_family_mut::<T>() {
            Some(family) => {
                family.components.borrow_mut().add(entity, component);
                T::group(&self, &entity);
            },
            None => {
                self.family_container.add_family::<T>(family_manager::Family::new());
                self.cleans.push(Box::new(|comp_manager: &ComponentManager, entity: Entity| {
                    comp_manager.get_components_mut::<T>().unwrap().remove(&entity);
                }));
                self.add_component::<T>(entity, component);
                return;
            },
        }
    }

    pub fn clean_components(&self, entity: Entity) {
        for func in self.cleans.iter() {
            func(&self, entity);
        }
    }
    
    pub fn get_components<T: Component>(&self) -> Option<View<T>> {
        match self.family_container.get_family::<T>() {
            Some(family) =>  {
                Some(family.components.borrow())
            },
            None => None,
        }
    }
    
    pub fn get_components_mut<T: Component>(&self) -> Option<ViewMut<T>> {
        match self.family_container.get_family::<T>() {
            Some(family) =>  {
                Some(family.components.borrow_mut())
            },
            None => None,
        }
    }

    pub fn has_component<T: Component>(&self, entity: &Entity) -> bool {
        match self.get_components::<T>() {
            Some(c) => c.contains(entity),
            None => false,
        }
    }
}

/// Used to register lone components.
#[allow(unused_macros)]
#[macro_export]
macro_rules! register_compontents {
    ($($component:ty),*) => {
        $(
        impl crate::ecs::Component for $component {
            fn group(_: &crate::ecs::cm::ComponentManager, _: &crate::ecs::Entity) { }
        }
        )*
    };
}

/// Used to register components that are grouped together fully owned.
#[allow(unused_macros)]
#[macro_export]
macro_rules! group {
    ($head:ty, $($tail:ty),+) => {
        group_rec!($head, $($tail),+;);
    };
}

/// Used to register a component that is grouped by other components.
#[allow(unused_macros)]
#[macro_export]
macro_rules! group_partial {
    ($head:ty; $($tail:ty),+) => {
        group_partial_imlp!($head; $($tail),+;);
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! group_imlp {
    ($head:ty, $($queue:ty),+) => {
        impl crate::ecs::Component for $head {
            fn group(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::Entity) {
                if $(cm.has_component::<$queue>(entity))&&+ {
                    cm.get_components_mut::<$head>().unwrap().group(entity);
                    $(
                    cm.get_components_mut::<$queue>().unwrap().group(entity);
                    )+
                }
            }
        }
    };
}


#[allow(unused_macros)]
#[macro_export]
macro_rules! group_partial_imlp {
    ($head:ty; $($queue:ty),+) => {
        impl crate::ecs::Groupable for $head {
            fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::Entity) {
                if $(cm.has_component::<$queue>(entity))&&+ {
                    cm.get_components_mut::<$head>().unwrap().group(entity);
                }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! group_rec {
    ($head:ty, $($queue:ty),+; $($done:ty),+) => {
        group_imlp!($head, $($queue),+, $($done),+);
        group_rec!($($queue),+; $($done),+, $head);
    };

    ($head:ty, $($queue:ty),+;) => {
        group_imlp!($head, $($queue),+);
        group_rec!($($queue),+; $head);
    };

    ($head:ty; $($done:ty),+) => {
        group_imlp!($head, $($done),+);
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