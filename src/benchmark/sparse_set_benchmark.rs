
extern crate test;
use self::test::Bencher;
use crate::sparse_set::SparseSet;
use crate::Entity;

const ITEM_AMOUNT: usize = 100000;

struct TestType { 
	data: usize 
}

#[bench]
fn sparse_set_add_one(b: &mut Bencher) {
	let mut set = SparseSet::<Entity, TestType>::new();

	b.iter(|| {
		set.add(&Entity::new(0, 0), TestType {data: 0});
	});
}

// #[bench]
// fn sparse_set_add_100k(b: &mut Bencher) {
// 	let mut set = SparseSet::<Entity, TestType>::new();

// 	b.iter(|| {
// 		for i in 0..ITEM_AMOUNT {
// 			set.add(&i, TestType {data: 0});
// 		}
// 	});
// }

#[bench]
fn sparse_set_remove_one(b: &mut Bencher) {
	let mut set = SparseSet::<Entity, TestType>::new();
	set.add(&Entity::new(0, 0), TestType {data: 0});

	b.iter(|| {
		set.remove(&Entity::new(0, 0));
	});
}

#[bench]
fn sparse_set_itarate_100k(b: &mut Bencher) {
	let mut set = SparseSet::<Entity, TestType>::new();
	for i in 0..ITEM_AMOUNT {
		set.add(&Entity::new(i as u32, 0), TestType {data: i as usize});
	}

	b.iter(|| {
        for i in 0..ITEM_AMOUNT {
			if let Some(mut comp) = set.component_at_mut(i) {
				comp.data = comp.data + 1;
			}
		}
	});
}

#[bench]
fn sparse_set_group_len30(b: &mut Bencher) {
	let mut set = SparseSet::<Entity, TestType>::new();
	for i in 0..30 {
		set.add(&Entity::new(i, 0), TestType {data: i as usize});
	}

	b.iter(|| {
		set.group(&Entity::new(25, 0));
	});
}

#[bench]
fn sparse_set_ungroup_len30(b: &mut Bencher) {
	let mut set = SparseSet::<Entity, TestType>::new();
	for i in 0..30 {
		set.add(&Entity::new(i, 0), TestType {data: i as usize});
	}
	set.group(&Entity::new(25, 0));
	set.group(&Entity::new(15, 0));
	set.group(&Entity::new(11, 0));
	set.group(&Entity::new(2, 0));

	b.iter(|| {
		set.ungroup(&Entity::new(15, 0));
	});
}