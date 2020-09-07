
use crate::sparse_set::SparseSet;
use crate::Entity;

struct TestType { 
	data: i8 
}

#[test]
fn empty_sparse_set() {
	let set = SparseSet::<TestType>::new();
	assert_eq!(set.len(), 0);
}

#[test]
fn sparse_set_add_1() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 0});
	assert_eq!(set.len(), 1);
}

#[test]
fn sparse_set_add_3() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 0});
	set.add(Entity::new(1, 0), TestType{data: 0});
	set.add(Entity::new(2, 0), TestType{data: 0});
	assert_eq!(set.len(), 3);
}

#[test]
fn sparse_set_add_same() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 0});
	set.add(Entity::new(0, 0), TestType{data: 0});
	set.add(Entity::new(0, 0), TestType{data: 0});
	assert_eq!(set.len(), 1);
}

#[test]
fn sparse_set_get_by_entityid() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});
	set.add(Entity::new(1, 0), TestType{data: 3});
	set.add(Entity::new(2, 0), TestType{data: 4});
	assert_eq!(set.get(&Entity::new(1, 0)).unwrap().data, 3);
	assert_eq!(set.get(&Entity::new(2, 0)).unwrap().data, 4);
	assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
}

#[test]
fn sparse_set_data_ordered() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});
	set.add(Entity::new(15, 0), TestType{data: 3});
	set.add(Entity::new(3, 0), TestType{data: 4});
	assert_eq!(set.component_at(0).unwrap().data, 7);
	assert_eq!(set.component_at(1).unwrap().data, 3);
	assert_eq!(set.component_at(2).unwrap().data, 4);

	assert_eq!(*set.entity_at(0).unwrap(), Entity::new(0, 0));
	assert_eq!(*set.entity_at(1).unwrap(), Entity::new(15, 0));
	assert_eq!(*set.entity_at(2).unwrap(), Entity::new(3, 0));

	assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
	assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
	assert_eq!(set.get(&Entity::new(3, 0)).unwrap().data, 4);
}

#[test]
fn sparse_set_group() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});
	set.add(Entity::new(15, 0), TestType{data: 3});
	set.add(Entity::new(3, 0), TestType{data: 4});
	set.add(Entity::new(9, 0), TestType{data: 25});
	set.add(Entity::new(5, 0), TestType{data: 44});
	set.group(&Entity::new(15, 0));
	set.group(&Entity::new(9, 0));

	assert_eq!(set.get_group_size(), 2);

	assert_eq!(set.component_at(0).unwrap().data, 3);
	assert_eq!(set.component_at(1).unwrap().data, 25);
	assert_eq!(set.component_at(2).unwrap().data, 4);
	assert_eq!(set.component_at(3).unwrap().data, 7);
	assert_eq!(set.component_at(4).unwrap().data, 44);

	assert_eq!(*set.entity_at(0).unwrap(), Entity::new(15, 0));
	assert_eq!(*set.entity_at(1).unwrap(), Entity::new(9, 0));
	assert_eq!(*set.entity_at(2).unwrap(), Entity::new(3, 0));
	assert_eq!(*set.entity_at(3).unwrap(), Entity::new(0, 0));
	assert_eq!(*set.entity_at(4).unwrap(), Entity::new(5, 0));
	
	assert_eq!(set.get(&Entity::new(9, 0)).unwrap().data, 25);
	assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
	assert_eq!(set.get(&Entity::new(3, 0)).unwrap().data, 4);
	assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
	assert_eq!(set.get(&Entity::new(5, 0)).unwrap().data, 44);
}

#[test]
fn sparse_set_group_efficien_edge_case() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});
	set.add(Entity::new(15, 0), TestType{data: 3});
	set.group(&Entity::new(0, 0));

	assert_eq!(set.get_group_size(), 1);

	assert_eq!(set.component_at(0).unwrap().data, 7);
	assert_eq!(set.component_at(1).unwrap().data, 3);

	assert_eq!(*set.entity_at(0).unwrap(), Entity::new(0, 0));
	assert_eq!(*set.entity_at(1).unwrap(), Entity::new(15, 0));
	
	assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
	assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
}

#[test]
fn sparse_set_double_group() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});
	set.add(Entity::new(15, 0), TestType{data: 3});
	set.add(Entity::new(3, 0), TestType{data: 4});
	set.add(Entity::new(9, 0), TestType{data: 25});
	set.add(Entity::new(5, 0), TestType{data: 44});
	set.group(&Entity::new(15, 0));
	set.group(&Entity::new(9, 0));
	set.group(&Entity::new(15, 0));

	assert_eq!(set.get_group_size(), 2);

	assert_eq!(set.component_at(0).unwrap().data, 3);
	assert_eq!(set.component_at(1).unwrap().data, 25);
	assert_eq!(set.component_at(2).unwrap().data, 4);
	assert_eq!(set.component_at(3).unwrap().data, 7);
	assert_eq!(set.component_at(4).unwrap().data, 44);

	assert_eq!(*set.entity_at(0).unwrap(), Entity::new(15, 0));
	assert_eq!(*set.entity_at(1).unwrap(), Entity::new(9, 0));
	assert_eq!(*set.entity_at(2).unwrap(), Entity::new(3, 0));
	assert_eq!(*set.entity_at(3).unwrap(), Entity::new(0, 0));
	assert_eq!(*set.entity_at(4).unwrap(), Entity::new(5, 0));
	// assert_eq!(*set.entity_at(0).unwrap(), 15);
	// assert_eq!(*set.entity_at(1).unwrap(), 9);
	// assert_eq!(*set.entity_at(2).unwrap(), 3);
	// assert_eq!(*set.entity_at(3).unwrap(), 0);
	// assert_eq!(*set.entity_at(4).unwrap(), 5);
	
	assert_eq!(set.get(&Entity::new(9, 0)).unwrap().data, 25);
	assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
	assert_eq!(set.get(&Entity::new(3, 0)).unwrap().data, 4);
	assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
	assert_eq!(set.get(&Entity::new(5, 0)).unwrap().data, 44);
}

#[test]
fn sparse_set_ungroup() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});
	set.add(Entity::new(15, 0), TestType{data: 3});
	set.add(Entity::new(3, 0), TestType{data: 4});
	set.add(Entity::new(9, 0), TestType{data: 25});
	set.add(Entity::new(5, 0), TestType{data: 44});
	set.group(&Entity::new(15, 0));
	set.group(&Entity::new(9, 0));
	set.ungroup(&Entity::new(15, 0));
	assert_eq!(set.component_at(0).unwrap().data, 25);
	assert_eq!(set.component_at(1).unwrap().data, 3);
	assert_eq!(set.component_at(2).unwrap().data, 4);
	assert_eq!(set.component_at(3).unwrap().data, 7);
	assert_eq!(set.component_at(4).unwrap().data, 44);

	assert_eq!(*set.entity_at(0).unwrap(), Entity::new(9, 0));
	assert_eq!(*set.entity_at(1).unwrap(), Entity::new(15, 0));
	assert_eq!(*set.entity_at(2).unwrap(), Entity::new(3, 0));
	assert_eq!(*set.entity_at(3).unwrap(), Entity::new(0, 0));
	assert_eq!(*set.entity_at(4).unwrap(), Entity::new(5, 0));
	
	assert_eq!(set.get(&Entity::new(9, 0)).unwrap().data, 25);
	assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
	assert_eq!(set.get(&Entity::new(3, 0)).unwrap().data, 4);
	assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
	assert_eq!(set.get(&Entity::new(5, 0)).unwrap().data, 44);

	assert_eq!(set.get_group_size(), 1);
}

#[test]
fn sparse_set_ungroup_efficien_edge_case() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});
	set.add(Entity::new(15, 0), TestType{data: 3});
	set.group(&Entity::new(0, 0));
	set.ungroup(&Entity::new(0, 0));

	assert_eq!(set.get_group_size(), 0);

	assert_eq!(set.component_at(0).unwrap().data, 7);
	assert_eq!(set.component_at(1).unwrap().data, 3);

	assert_eq!(*set.entity_at(0).unwrap(), Entity::new(0, 0));
	assert_eq!(*set.entity_at(1).unwrap(), Entity::new(15, 0));
	
	assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
	assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
}

#[test]
fn sparse_set_double_ungroup() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});
	set.add(Entity::new(15, 0), TestType{data: 3});
	set.add(Entity::new(3, 0), TestType{data: 4});
	set.add(Entity::new(9, 0), TestType{data: 25});
	set.add(Entity::new(5, 0), TestType{data: 44});
	set.group(&Entity::new(15, 0));
	set.group(&Entity::new(9, 0));
	set.ungroup(&Entity::new(15, 0));
	set.ungroup(&Entity::new(15, 0));
	assert_eq!(set.component_at(0).unwrap().data, 25);
	assert_eq!(set.component_at(1).unwrap().data, 3);
	assert_eq!(set.component_at(2).unwrap().data, 4);
	assert_eq!(set.component_at(3).unwrap().data, 7);
	assert_eq!(set.component_at(4).unwrap().data, 44);

	assert_eq!(*set.entity_at(0).unwrap(), Entity::new(9, 0));
	assert_eq!(*set.entity_at(1).unwrap(), Entity::new(15, 0));
	assert_eq!(*set.entity_at(2).unwrap(), Entity::new(3, 0));
	assert_eq!(*set.entity_at(3).unwrap(), Entity::new(0, 0));
	assert_eq!(*set.entity_at(4).unwrap(), Entity::new(5, 0));
	
	assert_eq!(set.get(&Entity::new(9, 0)).unwrap().data, 25);
	assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
	assert_eq!(set.get(&Entity::new(3, 0)).unwrap().data, 4);
	assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
	assert_eq!(set.get(&Entity::new(5, 0)).unwrap().data, 44);

	assert_eq!(set.get_group_size(), 1);
}

#[test]
fn sparse_set_remove() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});
	set.add(Entity::new(15, 0), TestType{data: 3});
	set.add(Entity::new(3, 0), TestType{data: 4});
	set.add(Entity::new(9, 0), TestType{data: 25});
	set.add(Entity::new(5, 0), TestType{data: 44});

	set.remove(&Entity::new(3, 0));

	assert_eq!(set.len(), 4);

	assert_eq!(set.component_at(0).unwrap().data, 7);
	assert_eq!(set.component_at(1).unwrap().data, 3);
	assert_eq!(set.component_at(2).unwrap().data, 44);
	assert_eq!(set.component_at(3).unwrap().data, 25);

	assert_eq!(*set.entity_at(0).unwrap(), Entity::new(0, 0));
	assert_eq!(*set.entity_at(1).unwrap(), Entity::new(15, 0));
	assert_eq!(*set.entity_at(2).unwrap(), Entity::new(5, 0));
	assert_eq!(*set.entity_at(3).unwrap(), Entity::new(9, 0));
	
	assert_eq!(set.get(&Entity::new(9, 0)).unwrap().data, 25);
	assert_eq!(set.get(&Entity::new(15, 0)).unwrap().data, 3);
	//assert_eq!(set.get(&3), None::<&TestType>);
	assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
	assert_eq!(set.get(&Entity::new(5, 0)).unwrap().data, 44);
}

#[test]
fn sparse_set_remove_one() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});

	set.remove(&Entity::new(0, 0));

	assert_eq!(set.len(), 0);
}

#[test]
fn sparse_set_remove_last() {
	let mut set = SparseSet::<TestType>::new();
	set.add(Entity::new(0, 0), TestType{data: 7});
	set.add(Entity::new(2, 0), TestType{data: 8});

	set.remove(&Entity::new(2, 0));

	assert_eq!(set.len(), 1);

	assert_eq!(set.component_at(0).unwrap().data, 7);

	assert_eq!(*set.entity_at(0).unwrap(), Entity::new(0, 0));
	
	assert_eq!(set.get(&Entity::new(0, 0)).unwrap().data, 7);
}