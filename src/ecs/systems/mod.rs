
use crate::ecs::ComponentView;

//Not stable yet
//pub trait func_type = 'static + Fn(ComponentView);

pub struct System {
    name: String,
    func_ptr: Box<dyn Fn(ComponentView)>  
}

impl System {
    pub fn new<F: 'static + Fn(ComponentView)>(name: &str, func: F) -> Self {
        Self {
            name: String::from(name),
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

