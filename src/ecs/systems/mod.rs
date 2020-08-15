
use crate::ecs::ComponentView;

//Not stable yet
//pub trait func_type = 'static + Fn(ComponentView);

pub struct System {
    func_ptr: Box<dyn Fn(ComponentView)>  
}

impl System {
    pub fn new<F: 'static + Fn(ComponentView)>(func: F) -> Self {
        Self {
            func_ptr: Box::new(func),
        }
    }
    pub fn run(&self, cm_ref: ComponentView) {
        (self.func_ptr)(cm_ref);
    }
}

// trait SystemTrait {
//     fn update(deltaTime: f32);

//     fn priority() -> i8;
// }

