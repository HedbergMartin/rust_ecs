#[macro_use]
mod ecs;

pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

pub struct HP {
    hp: i32,
}


fn main() {
    //let vec = Vec::<Comp>::new();
    // Todo get rid of mut
    let manager = ecs::Manager::new();

    for i in 0..100000 {
        let e = i;
        manager.add_component(&e, Position {x: i as i32, y:2, z: 2});
        manager.add_component(&e, HP {hp: 10});

    }
    
    //let now = std::time::Instant::now();
    //let hps = &(manager.get_components::<HP>().unwrap().dense_array).to_vec();
    //println!("Took {} ns\n", now.elapsed().as_nanos());
    //print!("Size {}", hps.len() * std::mem::size_of::<HP>());
    
    //print!("Number of position components: {}\n", positions.len());
    //impl_SystemTrait!(Position);
    //manager.run(|a: &ecs::sparse_set::SparseSet<Position>| {

    manager.schedule(|comp_manager: ecs::ComponentView| {
        for t in 1..10 {
            let now = std::time::Instant::now();
            match comp_manager.get_components_mut::<Position>() {
                Some(mut position) => {
                    match comp_manager.get_components::<HP>() {
                        Some(hps) => {
                            for (index, pos) in position.dense_array.iter_mut().enumerate() {
                                match hps.dense_array.get(index) {
                                    Some(hp) => {
                                        pos.x = pos.x + hp.hp;
                                    },
                                    None => {},
                                }
                                
                                //let hp = manager.get_component_copy::<HP>(&index);//hps.dense_array.get(index).unwrap();
                                //pos.y += hp.hp;
                            }
                        },
                        None => print!("Inner error"),
                    }
                },
                None => print!("Error!\n"),
            }
            println!("{}\n", now.elapsed().as_nanos());

            match comp_manager.get_components::<Position>() {
                Some(p) => {
                    for (i, pos) in p.dense_array.iter().enumerate() {
                        assert_eq!(pos.x as usize, i + t*10);
                    }
                },
                None => {},
            };
        }
    });

    print!("Wait!\n");
    std::thread::sleep(std::time::Duration::from_secs(3));

    manager.run(0);
    
    /*let now = std::time::Instant::now();
    for index in 0..manager.get_component_len::<Position>() {
        let hp = manager.get_component_val::<HP>(&index).unwrap();
        match manager.get_component_index_mut::<Position>(&index) {
            Some(p) => p.x = hp.hp + 100,
            None => {},
        }

    }
    println!("Second took {} ns\n", now.elapsed().as_nanos());*/

    /*
    let e: ecs::Entity = 0;
    manager.add_component(&e, Position {x: 2, y:2, z: 2});
    manager.add_component(&e, HP {hp: 10});
    let e: ecs::Entity = 1;
    manager.add_component(&e, Position {x: 3, y:3, z: 3});
    manager.add_component(&e, HP {hp: 20});
    let e: ecs::Entity = 2;
    manager.add_component(&e, Position {x: 4, y:5, z: 6});

    match manager.get_components::<Position>() {
        Some(v) => {
            print!("Number of position components: {}\n", v.len());
            for (index, comp) in v.dense_array.iter().enumerate() {
                print!("Position slot {} contains {:?}\n", index, comp);
            }
            //print!("Def: {}", v.get(0).unwrap().x);
        },
        None => print!("Error!\n"),
    }

    match manager.get_components::<HP>() {
        Some(v) => {
            print!("Number of hp components: {}\n", v.len());
            for (index, comp) in v.dense_array.iter().enumerate() {
                print!("HP slot {} contains {:?}\n", index, comp);
            }
        },
        None => print!("Error!\n"),
    }

    let ent = 1;
    
    match manager.get_component_mut::<Position>(&ent) {
        Some(c) => c.x = 100,
        None => print!("No position for component {} exsists", ent),
    }

    match manager.get_component::<Position>(&ent) {
        Some(c) => print!("Position for entity {}: {:?}", ent, c),
        None => print!("No position for component {} exsists", ent),
    }*/
    
}