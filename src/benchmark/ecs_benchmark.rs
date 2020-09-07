
extern crate test;
use self::test::Bencher;
use crate::ecs;

const ITEM_AMOUNT: usize = 100000;

#[allow(dead_code)]
struct PosComp {
	x: i32,
	y: i32,
	z: i32,
}

#[allow(dead_code)]
struct VelComp {
	x: i32,
	y: i32,
	z: i32,
}

#[allow(dead_code)]
struct RenderComp {
	x: i32,
	y: i32,
	z: i32,
}

#[allow(dead_code)]
struct HPComp {
	hp: i32,
}

#[allow(dead_code)]
struct OtherComp {
	x: i32,
	y: i32,
	z: i32,
}

#[allow(dead_code)]
struct SomeOtherComp {
	x: i32,
	y: i32,
	z: i32,
}

group!(PosComp, RenderComp);
group!(VelComp);
group!(OtherComp, SomeOtherComp, HPComp);

#[bench]
fn ecs_add_diffrent_groups(b: &mut Bencher) {
	let manager = ecs::Manager::new();

	b.iter(|| {
		let entity = manager.add_entity();
		manager.add_component(entity, PosComp{x: 0, y: 0, z: 0,});
		manager.add_component(entity, VelComp{x: 0, y: 0, z: 0,});
	});
}

#[bench]
fn ecs_add_same_group(b: &mut Bencher) {
	let manager = ecs::Manager::new();

	b.iter(|| {
		let entity = manager.add_entity();
		manager.add_component(entity, PosComp{x: 0, y: 0, z: 0,});
		manager.add_component(entity, RenderComp{x: 0, y: 0, z: 0,});
	});
}

#[bench]
fn ecs_kill_one(b: &mut Bencher) {
	let manager = ecs::Manager::new();
	for i in 0..20 {
		let entity = manager.add_entity();
		manager.add_component(entity, PosComp{x: 0, y: 0, z: 0,});
		manager.add_component(entity, VelComp{x: 0, y: 0, z: 0,});
	}

	
	let entity = manager.add_entity();
	manager.add_component(entity, PosComp{x: 0, y: 0, z: 0,});
	manager.add_component(entity, VelComp{x: 0, y: 0, z: 0,});

	b.iter(|| {
		manager.kill_entity(entity);
	});
}

/* 
#[bench]
fn cm_add_grouped_random(b: &mut Bencher) {
	let mut m = ComponentManager::new();
	for i in 0..11 {
		if (i < 4) {
			m.add_component(Entity::new(i, 0), GroupedPos{x: 0, y: 0, z: 0,});
		}
		if (i % 2 == 0) {
			m.add_component(Entity::new(i, 0), GroupedVel{x: 0, y: 0, z: 0,});
		}
	}

	// Pos will look like: 0 2 1 3
	// Vel will look like: 0 2 4 6 8 10

	b.iter(|| {
		m.add_component(Entity::new(6, 0), GroupedPos{x: 0, y: 0, z: 0,});
	});
} */