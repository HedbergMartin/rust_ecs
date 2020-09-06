
extern crate test;
use self::test::Bencher;
use crate::sparse_set::SparseSet;

const ITEM_AMOUNT: usize = 100000;

struct TestType { 
	data: usize 
}

#[bench]
fn sparse_set_add_one(b: &mut Bencher) {
	let mut set = SparseSet::<TestType>::new();

	b.iter(|| {
		set.add(0, TestType {data: 0});
	});
}

// #[bench]
// fn sparse_set_add_100k(b: &mut Bencher) {
// 	let mut set = SparseSet::<TestType>::new();

// 	b.iter(|| {
// 		for i in 0..ITEM_AMOUNT {
// 			set.add(i, TestType {data: 0});
// 		}
// 	});
// }

#[bench]
fn sparse_set_remove_one(b: &mut Bencher) {
	let mut set = SparseSet::<TestType>::new();
	set.add(0, TestType {data: 0});

	b.iter(|| {
		set.remove(&0);
	});
}

#[bench]
fn sparse_set_itarate_100k(b: &mut Bencher) {
	let mut set = SparseSet::<TestType>::new();
	for i in 0..ITEM_AMOUNT {
		set.add(i, TestType {data: i});
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
	let mut set = SparseSet::<TestType>::new();
	for i in 0..30 {
		set.add(i, TestType {data: i});
	}

	b.iter(|| {
		set.group(&25);
	});
}

#[bench]
fn sparse_set_ungroup_len30(b: &mut Bencher) {
	let mut set = SparseSet::<TestType>::new();
	for i in 0..30 {
		set.add(i, TestType {data: i});
	}
	set.group(&25);
	set.group(&15);
	set.group(&11);
	set.group(&2);

	b.iter(|| {
		set.ungroup(&15);
	});
}