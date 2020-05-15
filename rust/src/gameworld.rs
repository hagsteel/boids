use gdextras::input::InputEventExt;
use gdnative::{
    godot_error, godot_wrap_method, godot_wrap_method_inner, godot_wrap_method_parameter_count,
    methods, NativeClass, InputEvent, Node2D, Rect2, Vector2
};
use legion::prelude::*;
use rand::prelude::*;
use crate::spawner;

const BOID_COUNT: usize = 30;

fn physics_systems() -> Schedule {
    let schedule = Schedule::builder();

    // Add boids systems here

    schedule.build()
}

// -----------------------------------------------------------------------------
//     - Resources -
// -----------------------------------------------------------------------------
pub struct Delta(pub f32);

#[derive(Debug, Clone, Copy)]
pub struct Viewport(pub Rect2);

impl Viewport {
    fn from_vec2(size: Vector2) -> Self {
        let origin = size / 2.;
        let rect = Rect2::new(-origin.to_point(), size.to_size());
        Self(rect)
    }
}



// -----------------------------------------------------------------------------
//     - Godot node -
// -----------------------------------------------------------------------------

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GameWorld {
    world: World,
    physics: Schedule,
    resources: Resources,
}

#[methods]
impl GameWorld {
    pub fn _init(_owner: Node2D) -> Self {
        let mut resources = Resources::default();
        resources.insert(Delta(0.));

        let physics = physics_systems();

        Self { 
            world: Universe::new().create_world(),
            resources,
            physics,
        }
    }

    #[export]
    pub unsafe fn _ready(&mut self, mut owner: Node2D) {
        let mut rng = thread_rng();

        // Add viewport rect
        let size = owner.get_viewport().unwrap().get_size();
        let viewport = Viewport::from_vec2(size);
        self.resources.insert(viewport);

        for _ in 0..BOID_COUNT {
            let mut boid = spawner::spawn_boid();
            let x = rng.gen_range(viewport.0.min_x(), viewport.0.max_x());
            let y = rng.gen_range(viewport.0.min_y(), viewport.0.max_y());

            let pos = Vector2::new(x, y);
            owner.add_child(Some(boid.to_node()), false);
            boid.set_global_position(pos);


            let velocity = Vector2::new(
                rng.gen_range(-500., 500.),
                rng.gen_range(-500., 500.),
            ).normalize() * 500f32;

            self.world.insert(
                (),
                Some((
                    // Boid(boid)
                ))
            );
        }
    }

    #[export]
    pub fn _unhandled_input(&self, owner: Node2D, event: InputEvent) {
        if event.action_pressed("ui_cancel") {
            unsafe { owner.get_tree().map(|mut tree| tree.quit(0)) };
        }
    }

    #[export]
    pub fn _physics_process(&mut self, owner: Node2D, delta: f64) {
        self.resources.get_mut::<Delta>().map(|mut d| d.0 = delta as f32);
        self.physics.execute(&mut self.world, &mut self.resources);
    }
}