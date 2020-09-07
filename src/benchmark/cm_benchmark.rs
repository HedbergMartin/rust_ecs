
extern crate test;
use self::test::Bencher;
use crate::Entity;

use crate::cm::*;

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
	m.add_component(Entity::new(0, 0), GroupedPos{x: 0, y: 0, z: 0,});

	b.iter(|| {
		m.add_component(Entity::new(1, 0), GroupedPos{x: 0, y: 0, z: 0,});
	});
}

#[bench]
fn cm_add_grouped_both(b: &mut Bencher) {
	let mut m = ComponentManager::new();
	m.add_component(Entity::new(0, 0), GroupedPos{x: 0, y: 0, z: 0,});
	m.add_component(Entity::new(0, 0), GroupedVel{x: 0, y: 0, z: 0,});

	b.iter(|| {
		m.add_component(Entity::new(1, 0), GroupedPos{x: 0, y: 0, z: 0,});
		m.add_component(Entity::new(1, 0), GroupedVel{x: 0, y: 0, z: 0,});
	});
}

#[bench]
fn cm_add_grouped_random(b: &mut Bencher) {
	let mut m = ComponentManager::new();
	for i in 0..11 {
		if i < 4 {
			m.add_component(Entity::new(i, 0), GroupedPos{x: 0, y: 0, z: 0,});
		}
		if i % 2 == 0 {
			m.add_component(Entity::new(i, 0), GroupedVel{x: 0, y: 0, z: 0,});
		}
	}

	// Pos will look like: 0 2 1 3
	// Vel will look like: 0 2 4 6 8 10

	b.iter(|| {
		m.add_component(Entity::new(6, 0), GroupedPos{x: 0, y: 0, z: 0,});
	});
}