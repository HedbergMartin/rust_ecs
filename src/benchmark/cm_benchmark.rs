
extern crate test;
use self::test::Bencher;


use crate::cm::*;

const ITEM_AMOUNT: usize = 100000;

#[allow(dead_code)]
struct GroupedPos {
	x: i32,
	y: i32,
	z: i32,
}

#[allow(dead_code)]
struct GroupedVel {
	x: i32,
	y: i32,
	z: i32,
}

group!(GroupedPos, GroupedVel);

#[bench]
fn cm_add_grouped_one(b: &mut Bencher) {
	let mut m = ComponentManager::new();
	m.add_component(0, GroupedPos{x: 0, y: 0, z: 0,});

	b.iter(|| {
		m.add_component(1, GroupedPos{x: 0, y: 0, z: 0,});
	});
}

#[bench]
fn cm_add_grouped_both(b: &mut Bencher) {
	let mut m = ComponentManager::new();
	m.add_component(0, GroupedPos{x: 0, y: 0, z: 0,});
	m.add_component(0, GroupedVel{x: 0, y: 0, z: 0,});

	b.iter(|| {
		m.add_component(1, GroupedPos{x: 0, y: 0, z: 0,});
		m.add_component(1, GroupedVel{x: 0, y: 0, z: 0,});
	});
}

#[bench]
fn cm_add_grouped_random(b: &mut Bencher) {
	let mut m = ComponentManager::new();
	for i in 0..10 {
		if (i < 4) {
			m.add_component(i, GroupedPos{x: 0, y: 0, z: 0,});
		}
		if (i % 2 == 0) {
			m.add_component(i, GroupedVel{x: 0, y: 0, z: 0,});
		}
	}

	// Pos will look like: 0 2 1 3
	// Vel will look like: 0 2 4 6 8 10

	b.iter(|| {
		m.add_component(6, GroupedPos{x: 0, y: 0, z: 0,});
	});
}