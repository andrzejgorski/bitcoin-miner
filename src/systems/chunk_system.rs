use amethyst::{
    core::{
        math::Vector3,
        Transform,
    },
    ecs::{
        Entities, Join, Read, ReadStorage,
        System, WriteStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::camera::{ActiveCamera, Camera},
};

use crate::states::Chunk;
use crate::states::Map;
use crate::states::GameState;

#[derive(Default)]
pub struct ChunkSystem;
impl<'s> System<'s> for ChunkSystem {
    type SystemData = (
        Read<'s, ActiveCamera>,
        Entities<'s>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (active_camera, entities, cameras, transforms,): Self::SystemData) {
        //if x_move != 0.0 || y_move != 0.0 || z_move != 0.0 || z_move_scale != 0.0 {
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((_, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
    
                for chunk in crate::states::GameState.map.chunks{
                 }
                if camera_transform.local_y -

                //camera_transform.local_y = 1.0 
            }
        //}

    }
}