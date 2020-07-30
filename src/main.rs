mod ecs;

pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

pub struct HP {
    hp: i32,
}

trait FamilyTrait {
}

struct Family<T> {
    testId: i32,
    components: Vec<T>,
}

impl<T> FamilyTrait for Family<T> {

}

trait Behaviour {
    fn name(&self) -> &'static str;
    fn doBehaviour(&self);
}

struct SomeBehaviour;

impl Behaviour for SomeBehaviour {
    fn name(&self) -> &'static str {
        "Some Behaviour"
    }

    fn doBehaviour(&self) {
        println!("Doing something...");
    }
}

struct Container {
    families: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>,
}

impl Container {
    fn new() -> Container
    where {
        Container { families: std::collections::HashMap::new() }
    }

    fn add<T: std::any::Any>(&mut self, family: Family<T>)
    where T: std::any::Any, {
        print!("Added type of {:?}\n", std::any::TypeId::of::<T>());
        self.families.insert(std::any::TypeId::of::<T>(), Box::new(family));
    }

    fn get<T: std::any::Any>(&mut self) -> Option<& Family<T>> {
        //if let Some(b) = self.families.iter().find(|b| b.downcast_ref::<T>().map(|x| x.name()) == Some(name)) {
        print!("Get type of {:?}\n", std::any::TypeId::of::<T>());
        if let Some(b) = self.families.get(&(std::any::TypeId::of::<T>())) {
            print!("In here\n");
            return b.downcast_ref::<Family<T>>();
        }
        None
    }
}

fn main() {
    //let vec = Vec::<Comp>::new();
    let id = std::any::TypeId::of::<Position>();
    let id2 = std::any::TypeId::of::<Position>();
    let manager = ecs::new();
    manager.add_component::<Position>();
    println!("Hello, world! {:?} {:?}", id, id2);

    /*let mut v = Vec::<Box<dyn FamilyTrait>>::new();
    let fam = Family{ components: Vec::<Position>::new() };
    v.push(Box::new(fam));
    let fam = Family{ components: Vec::<HP>::new() };
    v.push(Box::new(fam));
*/
    // let out: Family<HP> = v.get(1);

    
    let mut container = Container::new();
    container.add(Family{ components: Vec::<Position>::new(), testId: 200});
    container.add(Family{ components: Vec::<HP>::new(), testId: -3});
    let someBehaviour = container.get::<HP>().unwrap();
    println!("Got behaviour: {}", someBehaviour.testId);
}

struct TypeMap {
    //map: std::collections::HashMap<u64, >
}