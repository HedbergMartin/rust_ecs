
//EXPERIMENTAL: trait aliases
//#![feature(trait_alias)]

#[macro_use]
mod ecs;

#[derive(Debug)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

pub struct HP {
    hp: i32,
}

group!(Position, HP);

fn main() {
    let manager = ecs::Manager::new();

    manager.register_task("Render", |comp_manager: ecs::ComponentView| {
        match comp_manager.get_components_mut::<Position>() {
            Some(mut position) => {
                match comp_manager.get_components::<HP>() {
                    Some(hps) => {
                        for index in 0..position.get_group() {
                            //Safe because they are grouped
                            let pos = position.component_at_mut(index).unwrap();
                            let hp = hps.component_at(index).unwrap();

                            pos.x = pos.x + hp.hp;
                        }
                    },
                    None => print!("Inner error"),
                }
            },
            None => print!("Error!\n"),
        }
    });

    manager.run_task("Render");
    
}