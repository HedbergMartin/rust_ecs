mod family_manager;

use crate::sparse_set;
use crate::Entity;

pub type View<'l, T> = std::cell::Ref<'l, sparse_set::SparseSet<T>>;
pub type ViewMut<'l, T> = std::cell::RefMut<'l, sparse_set::SparseSet<T>>;

pub trait Component: 'static {
    fn group(cm: &ComponentManager, entity: &Entity);
}

///
/// Sub manager to handle component part of the ecs.
/// 
pub struct ComponentManager {
    family_container: family_manager::Container,
	cleans: Vec<Box<dyn Fn(&ComponentManager, Entity)>>,
}

impl ComponentManager {
    pub(crate) fn new() -> Self {
        ComponentManager {
            family_container: family_manager::Container::new(),
			cleans: Vec::new(),
        }
    }

    ///
    /// Adds a new component to an entity.
    /// 
    /// # Panics
    /// 
    /// Panics if any other thread adds or works with components currently.
    ///
    /// # Examples
    ///
    /// ```
    /// struct Comp {}
    /// 
    /// !register_components(Comp);
    /// 
    /// let manager = rust_ecs::Manager::new();
    /// 
    /// let entity = manager.add_entity();
    /// manager.get_comp_manager_mut().add_component(entity, Comp {});
    /// ```
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

    pub(crate) fn clean_components(&self, entity: Entity) {
        for func in self.cleans.iter() {
            func(&self, entity);
        }
    }

    ///
    /// Gets the sparse_set of a certain component.
    /// 
    /// # Panics
    /// 
    /// Panics if any other thread adds the same component or borrows the same component as mutable.
    ///
    /// # Examples
    ///
    /// ```
    /// struct Comp {}
    /// 
    /// !register_components(Comp);
    /// 
    /// let manager = rust_ecs::Manager::new();
    /// 
    /// let entity = manager.add_entity();
    /// manager.add_component(entity, Comp {});
    /// let comp_manager = manager.get_comp_manager();
    /// 
    /// comp_manager.get_component::<Comp>();
    /// ```
    pub fn get_components<T: Component>(&self) -> Option<View<T>> {
        match self.family_container.get_family::<T>() {
            Some(family) =>  {
                Some(family.components.borrow())
            },
            None => None,
        }
    }

    ///
    /// Gets the mutable sparse_set of a certain component.
    /// 
    /// # Panics
    /// 
    /// Panics if any other thread adds or borrows the same component.
    ///
    /// # Examples
    ///
    /// ```
    /// struct Comp {}
    /// 
    /// !register_components(Comp);
    /// 
    /// let manager = rust_ecs::Manager::new();
    /// 
    /// let entity = manager.add_entity();
    /// manager.add_component(entity, Comp {});
    /// let comp_manager = manager.get_comp_manager();
    /// 
    /// comp_manager.get_component_mut::<Comp>();
    /// ```
    pub fn get_components_mut<T: Component>(&self) -> Option<ViewMut<T>> {
        match self.family_container.get_family::<T>() {
            Some(family) =>  {
                Some(family.components.borrow_mut())
            },
            None => None,
        }
    }

    ///
    /// Checks if a entity has the given component
    /// 
    /// # Panics
    /// 
    /// Panics if any other thread adds the same component or borrows the same component as mutable.
    /// Will be solved soon when Manager becomes threadsafe.
    ///
    /// # Examples
    ///
    /// ```
    /// struct Comp {}
    /// 
    /// !register_components(Comp);
    /// 
    /// let manager = rust_ecs::Manager::new();
    /// 
    /// let entity = manager.add_entity();
    /// manager.add_component(entity, Comp {});
    /// let comp_manager = manager.get_comp_manager();
    /// 
    /// assert!(comp_manager.has_component::<Comp>(entity));
    /// ```
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
macro_rules! register_components {
    ($($component:ty),*) => {
        $(
        impl $crate::Component for $component {
            fn group(_: &$crate::cm::ComponentManager, _: &$crate::Entity) { }
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
macro_rules! group_imlp {
    ($head:ty, $($queue:ty),+) => {
        impl $crate::Component for $head {
            fn group(cm: &$crate::cm::ComponentManager, entity: &$crate::Entity) {
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
macro_rules! group_partial_imlp {
    ($head:ty; $($queue:ty),+) => {
        impl $crate::Component for $head {
            fn sort(cm: &$crate::cm::ComponentManager, entity: &$crate::Entity) {
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