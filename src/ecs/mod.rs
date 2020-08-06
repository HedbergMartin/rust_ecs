//mod sparse_set;

pub mod sparse_set;
mod component_manager;

pub type Entity = usize;

pub struct Manager {
    entities: Vec<i32>,
    pub component_manager: component_manager::ComponentManager,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            entities: Vec::new(),
            component_manager: component_manager::ComponentManager::new(),
        }
    }

    


    /*pub fn get_components_mut<T: std::any::Any>(&mut self) -> Option<&mut sparse_set::SparseSet<T>> {
        print!("Getting components of type {}!\n", std::any::type_name::<T>());
        match self.container.get_mut::<T>() {
            Some(family) =>  Some(&mut family.components),
            None => None,
        }
    }

    pub fn get_component_copy<T: std::any::Any + std::marker::Copy>(&self, index: &usize) -> Option<T> {
        print!("Getting components of type {}!\n", std::any::type_name::<T>());
        match self.container.get::<T>() {
            Some(family) =>  {
                match family.components.dense_array.get(*index) {
                    Some(comp) => Some(*comp),
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn get_components_ownership<T: std::any::Any>(&mut self) -> Option<sparse_set::SparseSet<T>> {
        print!("Getting components ownership of type {}!\n", std::any::type_name::<T>());
        match self.container.get_mut::<T>() {
            Some(family) => None,// Some(&mut family.components),
            None => None,
        }
    }*/


    /*pub fn get_component_len<T: std::any::Any>(&self) -> usize {
        match self.container.get::<T>() {
            Some(family) =>  family.components.len(),
            None => 0,
        }
    }
    
    pub fn get_component_mut<T: std::any::Any>(&mut self, entity: &Entity) -> Option<&mut T> {
        //print!("Getting component of type {} for entity with ID {}!\n", std::any::type_name::<T>(), entity);
        match self.container.get_mut::<T>() {
            Some(family) =>  family.components.get_mut(entity),
            None => None,
        }
    }*/

    /*pub fn get_component_index_mut<T: std::any::Any>(&mut self, index: &usize) -> Option<&mut T> {
        match self.container.get_mut::<T>() {
            Some(family) =>  family.components.dense_array.get_mut(*index),
            None => None,
        }
    }

    pub fn get_component_val<T: std::any::Any + Copy>(&self, index: &usize) -> Option<T> {
        match self.container.get::<T>() {
            Some(family) =>  Some(*family.components.dense_array.get(*index).unwrap()),
            None => None,
        }
    }*/

    /*pub fn update<T: std::any::Any, U: std::any::Any>(&mut self) {
        let now = std::time::Instant::now();
        match self.get_components_mut::<T>() {
            Some(position) => {
                match self.get_components::<U>() {
                    Some(hps) => {
                        for (index, pos) in position.dense_array.iter_mut().enumerate() {
                            let hp = hps.dense_array.get(index).unwrap();
                            //pos.y += hp.hp;
                        }
                    },
                    None => print!("Errore"),
                }
            },
            None => print!("Error!\n"),
        }
        println!("Took {} ns\n", now.elapsed().as_nanos());
    }*/
    
    /*pub fn get_component_mut2<T: std::any::Any, U: std::any::Any>(&mut self, entity: &Entity) -> (Option<&mut T>, Option<&mut U>) {
        match self.container.get_mut::<T>() {
            Some(family1) =>  match self.container.get_mut::<U>() {
                Some(family2) =>  (family1.components.get_mut(entity), family2.components.get_mut(entity)),
                None => (None, None),
            },
            None => (None, None),
        }
    }*/
}