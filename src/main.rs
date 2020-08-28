
//EXPERIMENTAL: trait aliases
//#![feature(trait_alias)]

#[macro_use]
mod ecs;

use crate::ecs::sparse_set::Groupable;

#[derive(Debug)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

pub struct HP {
    hp: i32,
}

// impl ecs::cm::Group for Position {
//     fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::sparse_set::Entity) {
//         //TODO unwrap?
//         // Will contain pos as it is the caller.
//         // print!("Sorting pos\n");
//         if cm.get_components_mut::<HP>().is_some() {
//             if cm.get_components_mut::<HP>().unwrap().contains(entity) {
//                 cm.get_components_mut::<Position>().unwrap().swap(entity);
//                 cm.get_components_mut::<HP>().unwrap().swap(entity);
//             }
//         }
//     }
// }

// impl ecs::cm::Group for HP {
//     fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::sparse_set::Entity) {
//         //TODO unwrap?
//         // print!("Sorting hps\n");
//         if cm.get_components_mut::<Position>().is_some() {
//             if cm.get_components_mut::<Position>().unwrap().contains(entity) {
//                 cm.get_components_mut::<HP>().unwrap().swap(entity);
//                 cm.get_components_mut::<Position>().unwrap().swap(entity);
//             }
//         }
//     }
// }

macro_rules! group {
    ($head:ty) => {
        impl ecs::cm::Group for $head {
            fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::sparse_set::Entity) {
                
            }
        }
    };
    ($head:ty, $($tail:ty),+) => {
        group_raw!($head, $($tail),+;);
        //TODO make all in tail head at least once
    };
}

macro_rules! group_raw {

    ($head:ty, $($queue:ty),+; $($done:ty),+) => {
        impl ecs::cm::Group for $head {
            fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::sparse_set::Entity) {
                //TODO unwrap?
                // print!("Sorting hps\n");
                //TODO not mut?
                if $(cm.contains::<$queue>(entity))&&+ && $(cm.contains::<$done>(entity))&&+ {
                    cm.get_components_mut::<$head>().unwrap().swap(entity);
                    $(
                    cm.get_components_mut::<$queue>().unwrap().swap(entity);
                    )+
                    $(
                    cm.get_components_mut::<$done>().unwrap().swap(entity);
                    )+
                }
            }
        }
        group_raw!($($queue),+; $($done),+, $head);
    };

    ($head:ty; $($done:ty),+) => {
        impl ecs::cm::Group for $head {
            fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::sparse_set::Entity) {
                //TODO unwrap?
                // print!("Sorting hps\n");
                //TODO not mut?
                if $(cm.contains::<$done>(entity))&&+ {
                    cm.get_components_mut::<$head>().unwrap().swap(entity);
                    $(
                    cm.get_components_mut::<$done>().unwrap().swap(entity);
                    )+
                }
            }
        }
    };
    
    ($head:ty, $($queue:ty),+; ) => {
        impl ecs::cm::Group for $head {
            fn sort(cm: &crate::ecs::cm::ComponentManager, entity: &crate::ecs::sparse_set::Entity) {
                //TODO unwrap?
                // print!("Sorting hps\n");
                //TODO not mut?
                if $(cm.contains::<$queue>(entity))&&+ {
                    cm.get_components_mut::<$head>().unwrap().swap(entity);
                    $(
                    cm.get_components_mut::<$queue>().unwrap().swap(entity);
                    )+
                }
            }
        }
        group_raw!($($queue),+; $head);
    };
}

group!(Position, HP);

fn main() {
    //let vec = Vec::<Comp>::new();
    // Todo get rid of mut
    let manager = ecs::Manager::new();

    // for i in 0..3 {

    // -------------
        
    let e1 = manager.add_entity();
    manager.add_component(&e1, Position {x: 2 as i32, y:2, z: 2});
    manager.add_component(&e1, HP {hp: 10});

    print!("Positions: \n");
    manager.print_components::<Position>();

    print!("HP: \n");
    manager.print_components::<HP>();
    print!("\n");

    // -------------
        
    let e2 = manager.add_entity();
    manager.add_component(&e2, Position {x: 2 as i32, y:2, z: 2});

    print!("Positions: \n");
    manager.print_components::<Position>();

    print!("HP: \n");
    manager.print_components::<HP>();
    print!("\n");

    // -------------
        
    let e3 = manager.add_entity();
    manager.add_component(&e3, HP {hp: 10});

    print!("Positions: \n");
    manager.print_components::<Position>();

    print!("HP: \n");
    manager.print_components::<HP>();
    print!("\n");

    // -------------
        
    manager.add_component(&e2, HP {hp: 10});

    print!("Positions: \n");
    manager.print_components::<Position>();

    print!("HP: \n");
    manager.print_components::<HP>();
    print!("\n");
    // }

    
    let a = 2;
    let b = Position {x: 2, y:34, z: 1 };

    //group!(&a);

    //let now = std::time::Instant::now();
    //let hps = &(manager.get_components::<HP>().unwrap().dense_array).to_vec();
    //println!("Took {} ns\n", now.elapsed().as_nanos());
    //print!("Size {}", hps.len() * std::mem::size_of::<HP>());
    
    //print!("Number of position components: {}\n", positions.len());
    //impl_SystemTrait!(Position);
    //manager.run(|a: &ecs::sparse_set::SparseSet<Position>| {

    manager.register_task("Group", |comp_manager: ecs::ComponentView| {
        match comp_manager.get_components_mut::<Position>() {
            Some(mut position) => {
                match comp_manager.get_components_mut::<HP>() {
                    Some(mut hps) => {
                        //for every unparsed 

                        // position.group(|entity: ecs::sparse_set::Entity| {
                        //     if hps.contains(&entity) {
                        //         hps.swap(&entity);
                        //         return true;
                        //     }
                        //     return false;
                        // });

                        // hps.group(|entity: ecs::sparse_set::Entity| {
                        //     if position.contains(&entity) {
                        //         position.swap(&entity);
                        //         return true;
                        //     }
                        //     return false;
                        // });
                        //Check unparsed for positions
                            // For every found, swap position and all in group (hp)

                        //Check unparsed for hp
                            // For every found, swap hp and all in group (position)

                        // position.group(|entity: ecs::sparse_set::Entity| {
                        //     hps.contains(&entity)
                        // });
                        // hps.group(|entity: ecs::sparse_set::Entity| {
                        //     position.contains(&entity)
                        // });
                    },
                    None => print!("Inner error"),
                }
            },
            None => print!("Error!\n"),
        }
    });

    //manager.run_task("Group");

    // manager.register_task("Render", |comp_manager: ecs::ComponentView| {
    //     for t in 1..10 {
    //         let now = std::time::Instant::now();
    //         match comp_manager.get_components_mut::<Position>() {
    //             Some(mut position) => {
    //                 match comp_manager.get_components::<HP>() {
    //                     Some(hps) => {
    //                         for (index, pos) in position.dense_array.iter_mut().enumerate() {
    //                             match hps.dense_array.get(index) {
    //                                 Some(hp) => {
    //                                     pos.x = pos.x + hp.hp;
    //                                 },
    //                                 None => {},
    //                             }
                                
    //                             //let hp = manager.get_component_copy::<HP>(&index);//hps.dense_array.get(index).unwrap();
    //                             //pos.y += hp.hp;
    //                         }
    //                     },
    //                     None => print!("Inner error"),
    //                 }
    //             },
    //             None => print!("Error!\n"),
    //         }
    //         println!("{}\n", now.elapsed().as_nanos());

    //         match comp_manager.get_components::<Position>() {
    //             Some(p) => {
    //                 for (i, pos) in p.dense_array.iter().enumerate() {
    //                     assert_eq!(pos.x as usize, i + t*10);
    //                 }
    //             },
    //             None => {},
    //         };
    //     }
    // });

    manager.register_task("Prnt",|comp_manager: ecs::ComponentView| {
        print!("Numerurehro\n");
    });

    //print!("Wait!\n");
    //std::thread::sleep(std::time::Duration::from_secs(3));

    //manager.run_task("Prnt");
    //std::thread::sleep(std::time::Duration::from_secs(3));

    //manager.run_task("Nope");
    //std::thread::sleep(std::time::Duration::from_secs(3));
    manager.run_task("Render");
    
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