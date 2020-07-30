mod ecs;

#[allow(dead_code)]
#[derive(Default)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct HP {
    hp: i32,
}

fn main() {
    //let vec = Vec::<Comp>::new();
    let mut manager = ecs::Manager::new();
    let e: ecs::Entity = 0;
    manager.add_component::<Position>(&e);
    manager.add_component::<HP>(&e);
    let e: ecs::Entity = 1;
    manager.add_component::<Position>(&e);
    manager.add_component::<HP>(&e);
    let e: ecs::Entity = 2;
    manager.add_component::<Position>(&e);

    match manager.get_components::<Position>() {
        Some(v) => {
            print!("Number of position components: {}\n", v.len());
        },
        None => print!("Error!\n"),
    }

    match manager.get_components::<HP>() {
        Some(v) => {
            print!("Number of hp components: {}\n", v.len());
        },
        None => print!("Error!\n"),
    }

    /*let mut v = Vec::<Box<dyn FamilyTrait>>::new();
    let fam = Family{ components: Vec::<Position>::new() };
    v.push(Box::new(fam));
    let fam = Family{ components: Vec::<HP>::new() };
    v.push(Box::new(fam));
*/
    // let out: Family<HP> = v.get(1);

    
    //let mut container = ecs::Container::new();
    //container.add(ecs::Family{ components: Vec::<Position>::new(), test_id: 200});
    //container.add(ecs::Family{ components: Vec::<HP>::new(), test_id: -3});
    //let fam = container.get::<HP>().unwrap();
    //println!("Got behaviour: {}", fam.test_id);
}