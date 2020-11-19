use amethyst::{
    core::{
        math::Vector3,
        Parent, Transform,
    },
    ecs::{
        Entities, Join, LazyUpdate,Entity, Read, ReadExpect, ReadStorage, System, 
    },
    input::{InputHandler, StringBindings},
    prelude::*,
    renderer::camera::{ActiveCamera, Camera},
    window::ScreenDimensions,
};

pub struct CameraSwitchSystem {
    pressed: bool,
    perspective: bool,
}
impl Default for CameraSwitchSystem {
    fn default() -> Self {
        Self {
            pressed: false,
            perspective: false,
        }
    }
}

impl<'s> System<'s> for CameraSwitchSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Parent>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (entities, lazy, active_camera, dimensions, cameras, transforms, parents, input): Self::SystemData,
    ) {
        if input.action_is_down("camera_switch").unwrap() {
            self.pressed = true;
        }
        if self.pressed && !input.action_is_down("camera_switch").unwrap() {
            self.pressed = false;

            // Lazily delete the old camera
            let mut camera_join = (&entities, &cameras, &transforms, &parents).join();
            let (old_camera_entity, _, _, old_parent) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
                .unwrap();
            let old_camera_entity = old_camera_entity;

            let new_parent = old_parent.entity;

            self.perspective = !self.perspective;
            let (new_camera, new_position) = if self.perspective {
                (
                    Camera::standard_3d(dimensions.width(), dimensions.height()),
                    Vector3::new(0.0, 0.0, 500.1),
                )
            } else {
                (
                    Camera::standard_2d(dimensions.width(), dimensions.height()),
                    Vector3::new(0.0, 0.0, 1.1),
                )
            };

            lazy.exec_mut(move |w| {
                let new_camera =
                    init_camera(w, new_parent, Transform::from(new_position), new_camera);

                w.fetch_mut::<ActiveCamera>().entity = Some(new_camera);

                w.delete_entity(old_camera_entity).unwrap();
            });
        }
    }
}

fn init_camera(world: &mut World, parent: Entity, transform: Transform, camera: Camera) -> Entity {
    world
        .create_entity()
        .with(transform)
        .with(Parent { entity: parent })
        .with(camera)
        .named("camera")
        .build()
}