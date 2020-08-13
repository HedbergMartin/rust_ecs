
use crate::ecs::ComponentView;

pub struct System<RunFunc>
where RunFunc : Fn(ComponentView) {
    func: RunFunc,
    //Todo ref to manager    
}

impl<RunFunc> System<RunFunc> 
where RunFunc: Fn(ComponentView) {
    pub fn new(func: RunFunc) -> Self {
        Self {
            func,
        }
    }

    pub fn run(&self, cm_ref: ComponentView) {
        (self.func)(cm_ref);
    }
}

// trait SystemTrait {
//     fn update(deltaTime: f32);

//     fn priority() -> i8;
// }

