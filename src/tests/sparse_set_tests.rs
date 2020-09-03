
use crate::sparse_set::SparseSet;

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
	set.add(0, TestType{data: 0});
	assert_eq!(set.len(), 1);
}

#[test]
fn sparse_set_add_3() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 0});
	set.add(1, TestType{data: 0});
	set.add(2, TestType{data: 0});
	assert_eq!(set.len(), 3);
}

#[test]
fn sparse_set_add_same() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 0});
	set.add(0, TestType{data: 0});
	set.add(0, TestType{data: 0});
	assert_eq!(set.len(), 1);
}

#[test]
fn sparse_set_get_by_entityid() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});
	set.add(1, TestType{data: 3});
	set.add(2, TestType{data: 4});
	assert_eq!(set.get(&1).unwrap().data, 3);
	assert_eq!(set.get(&2).unwrap().data, 4);
	assert_eq!(set.get(&0).unwrap().data, 7);
}

#[test]
fn sparse_set_data_ordered() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});
	set.add(15, TestType{data: 3});
	set.add(3, TestType{data: 4});
	assert_eq!(set.component_at(0).unwrap().data, 7);
	assert_eq!(set.component_at(1).unwrap().data, 3);
	assert_eq!(set.component_at(2).unwrap().data, 4);

	assert_eq!(*set.entity_at(0).unwrap(), 0);
	assert_eq!(*set.entity_at(1).unwrap(), 15);
	assert_eq!(*set.entity_at(2).unwrap(), 3);
}