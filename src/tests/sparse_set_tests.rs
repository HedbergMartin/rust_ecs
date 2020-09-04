
use crate::sparse_set::SparseSet;

#[derive(Debug)]
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

	assert_eq!(set.get(&0).unwrap().data, 7);
	assert_eq!(set.get(&15).unwrap().data, 3);
	assert_eq!(set.get(&3).unwrap().data, 4);
}

#[test]
fn sparse_set_group() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});
	set.add(15, TestType{data: 3});
	set.add(3, TestType{data: 4});
	set.add(9, TestType{data: 25});
	set.add(5, TestType{data: 44});
	set.group(&15);
	set.group(&9);

	assert_eq!(set.get_group_size(), 2);

	assert_eq!(set.component_at(0).unwrap().data, 3);
	assert_eq!(set.component_at(1).unwrap().data, 25);
	assert_eq!(set.component_at(2).unwrap().data, 4);
	assert_eq!(set.component_at(3).unwrap().data, 7);
	assert_eq!(set.component_at(4).unwrap().data, 44);

	assert_eq!(*set.entity_at(0).unwrap(), 15);
	assert_eq!(*set.entity_at(1).unwrap(), 9);
	assert_eq!(*set.entity_at(2).unwrap(), 3);
	assert_eq!(*set.entity_at(3).unwrap(), 0);
	assert_eq!(*set.entity_at(4).unwrap(), 5);
	
	assert_eq!(set.get(&9).unwrap().data, 25);
	assert_eq!(set.get(&15).unwrap().data, 3);
	assert_eq!(set.get(&3).unwrap().data, 4);
	assert_eq!(set.get(&0).unwrap().data, 7);
	assert_eq!(set.get(&5).unwrap().data, 44);
}

#[test]
fn sparse_set_group_efficien_edge_case() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});
	set.add(15, TestType{data: 3});
	set.group(&0);

	assert_eq!(set.get_group_size(), 1);

	assert_eq!(set.component_at(0).unwrap().data, 7);
	assert_eq!(set.component_at(1).unwrap().data, 3);

	assert_eq!(*set.entity_at(0).unwrap(), 0);
	assert_eq!(*set.entity_at(1).unwrap(), 15);
	
	assert_eq!(set.get(&0).unwrap().data, 7);
	assert_eq!(set.get(&15).unwrap().data, 3);
}

#[test]
fn sparse_set_double_group() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});
	set.add(15, TestType{data: 3});
	set.add(3, TestType{data: 4});
	set.add(9, TestType{data: 25});
	set.add(5, TestType{data: 44});
	set.group(&15);
	set.group(&9);
	set.group(&15);

	assert_eq!(set.get_group_size(), 2);

	assert_eq!(set.component_at(0).unwrap().data, 3);
	assert_eq!(set.component_at(1).unwrap().data, 25);
	assert_eq!(set.component_at(2).unwrap().data, 4);
	assert_eq!(set.component_at(3).unwrap().data, 7);
	assert_eq!(set.component_at(4).unwrap().data, 44);

	assert_eq!(*set.entity_at(0).unwrap(), 15);
	assert_eq!(*set.entity_at(1).unwrap(), 9);
	assert_eq!(*set.entity_at(2).unwrap(), 3);
	assert_eq!(*set.entity_at(3).unwrap(), 0);
	assert_eq!(*set.entity_at(4).unwrap(), 5);
	
	assert_eq!(set.get(&9).unwrap().data, 25);
	assert_eq!(set.get(&15).unwrap().data, 3);
	assert_eq!(set.get(&3).unwrap().data, 4);
	assert_eq!(set.get(&0).unwrap().data, 7);
	assert_eq!(set.get(&5).unwrap().data, 44);
}

#[test]
fn sparse_set_ungroup() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});
	set.add(15, TestType{data: 3});
	set.add(3, TestType{data: 4});
	set.add(9, TestType{data: 25});
	set.add(5, TestType{data: 44});
	set.group(&15);
	set.group(&9);
	set.ungroup(&15);
	assert_eq!(set.component_at(0).unwrap().data, 25);
	assert_eq!(set.component_at(1).unwrap().data, 3);
	assert_eq!(set.component_at(2).unwrap().data, 4);
	assert_eq!(set.component_at(3).unwrap().data, 7);
	assert_eq!(set.component_at(4).unwrap().data, 44);

	assert_eq!(*set.entity_at(0).unwrap(), 9);
	assert_eq!(*set.entity_at(1).unwrap(), 15);
	assert_eq!(*set.entity_at(2).unwrap(), 3);
	assert_eq!(*set.entity_at(3).unwrap(), 0);
	assert_eq!(*set.entity_at(4).unwrap(), 5);
	
	assert_eq!(set.get(&9).unwrap().data, 25);
	assert_eq!(set.get(&15).unwrap().data, 3);
	assert_eq!(set.get(&3).unwrap().data, 4);
	assert_eq!(set.get(&0).unwrap().data, 7);
	assert_eq!(set.get(&5).unwrap().data, 44);

	assert_eq!(set.get_group_size(), 1);
}

#[test]
fn sparse_set_ungroup_efficien_edge_case() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});
	set.add(15, TestType{data: 3});
	set.group(&0);
	set.ungroup(&0);

	assert_eq!(set.get_group_size(), 0);

	assert_eq!(set.component_at(0).unwrap().data, 7);
	assert_eq!(set.component_at(1).unwrap().data, 3);

	assert_eq!(*set.entity_at(0).unwrap(), 0);
	assert_eq!(*set.entity_at(1).unwrap(), 15);
	
	assert_eq!(set.get(&0).unwrap().data, 7);
	assert_eq!(set.get(&15).unwrap().data, 3);
}

#[test]
fn sparse_set_double_ungroup() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});
	set.add(15, TestType{data: 3});
	set.add(3, TestType{data: 4});
	set.add(9, TestType{data: 25});
	set.add(5, TestType{data: 44});
	set.group(&15);
	set.group(&9);
	set.ungroup(&15);
	set.ungroup(&15);
	assert_eq!(set.component_at(0).unwrap().data, 25);
	assert_eq!(set.component_at(1).unwrap().data, 3);
	assert_eq!(set.component_at(2).unwrap().data, 4);
	assert_eq!(set.component_at(3).unwrap().data, 7);
	assert_eq!(set.component_at(4).unwrap().data, 44);

	assert_eq!(*set.entity_at(0).unwrap(), 9);
	assert_eq!(*set.entity_at(1).unwrap(), 15);
	assert_eq!(*set.entity_at(2).unwrap(), 3);
	assert_eq!(*set.entity_at(3).unwrap(), 0);
	assert_eq!(*set.entity_at(4).unwrap(), 5);
	
	assert_eq!(set.get(&9).unwrap().data, 25);
	assert_eq!(set.get(&15).unwrap().data, 3);
	assert_eq!(set.get(&3).unwrap().data, 4);
	assert_eq!(set.get(&0).unwrap().data, 7);
	assert_eq!(set.get(&5).unwrap().data, 44);

	assert_eq!(set.get_group_size(), 1);
}

#[test]
fn sparse_set_remove() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});
	set.add(15, TestType{data: 3});
	set.add(3, TestType{data: 4});
	set.add(9, TestType{data: 25});
	set.add(5, TestType{data: 44});

	set.remove(&3);

	assert_eq!(set.len(), 4);

	assert_eq!(set.component_at(0).unwrap().data, 7);
	assert_eq!(set.component_at(1).unwrap().data, 3);
	assert_eq!(set.component_at(2).unwrap().data, 44);
	assert_eq!(set.component_at(3).unwrap().data, 25);

	assert_eq!(*set.entity_at(0).unwrap(), 0);
	assert_eq!(*set.entity_at(1).unwrap(), 15);
	assert_eq!(*set.entity_at(2).unwrap(), 5);
	assert_eq!(*set.entity_at(3).unwrap(), 9);
	
	assert_eq!(set.get(&9).unwrap().data, 25);
	assert_eq!(set.get(&15).unwrap().data, 3);
	//assert_eq!(set.get(&3), None::<&TestType>);
	assert_eq!(set.get(&0).unwrap().data, 7);
	assert_eq!(set.get(&5).unwrap().data, 44);
}

#[test]
fn sparse_set_remove_one() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});

	set.remove(&0);

	assert_eq!(set.len(), 0);
}

#[test]
fn sparse_set_remove_last() {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType{data: 7});
	set.add(2, TestType{data: 8});

	set.remove(&2);

	assert_eq!(set.len(), 1);

	assert_eq!(set.component_at(0).unwrap().data, 7);

	assert_eq!(*set.entity_at(0).unwrap(), 0);
	
	assert_eq!(set.get(&0).unwrap().data, 7);
}