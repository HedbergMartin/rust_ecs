
use crate::sparse_set::SparseSet;
use crate::Entity;

type Set<T> = SparseSet<Entity, T>;

pub trait Borrowable<'c> : Sized {
    fn borrow(cm: &'c crate::ComponentManager) -> Self;
}

pub struct View<'l, T: 'static> {
    set_ref: std::cell::Ref<'l, Set<T>>,
}

impl<'l, T: 'static> View<'l, T> {
    pub fn new(set_ref: std::cell::Ref<'l, Set<T>>) -> Self {
        Self {
            set_ref,
        }
    }
}

impl<'l, T: 'static + crate::Component> Borrowable<'l> for View<'l, T> {
    fn borrow(cm: &'l crate::ComponentManager) -> Self {
        cm.get_components::<T>().unwrap()
    }
}

impl<T: 'static> std::ops::Deref for View<'_, T> {
    type Target = Set<T>;

    fn deref(&self) -> &Self::Target {
        &self.set_ref
    }
}

pub struct ViewMut<'l, T: 'static> {
    set_ref: std::cell::RefMut<'l, Set<T>>,
}

impl<'l, T: 'static + crate::Component> Borrowable<'l> for ViewMut<'l, T> {
    fn borrow(cm: &'l crate::ComponentManager) -> Self {
        cm.get_components_mut::<T>().unwrap()
    }
}

impl<'l, T: 'static> ViewMut<'l, T> {
    pub fn new(set_ref: std::cell::RefMut<'l, Set<T>>) -> Self {
        Self {
            set_ref,
        }
    }
}

impl<'l, T: 'static> std::ops::Deref for ViewMut<'l, T> {
    type Target = Set<T>;

    fn deref(&self) -> &Self::Target {
        &self.set_ref
    }
}

impl<'l, T: 'static> std::ops::DerefMut for ViewMut<'l, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.set_ref
    }
}