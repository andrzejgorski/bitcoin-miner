use amethyst::{
    core::{
        math::Vector3,
        Transform,
    },
    ecs::{
        Entities, Join, Read, ReadStorage, World,
        System, WriteStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::camera::{ActiveCamera, Camera},
    tiles::{MortonEncoder, TileMap},
};

use crate::map_structure::*;

#[derive(Default)]
pub struct ChunkSystem;
impl<'s> System<'s> for ChunkSystem {
    type SystemData = (
        Read<'s, ActiveCamera>,
        Entities<'s>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Chunk>,
        ReadStorage<'s, TileMap<ExampleTile, MortonEncoder>>,
    );

    fn run(&mut self, (active_camera, entities, cameras, transforms, chunks, tiles): Self::SystemData) {
        //if x_move != 0.0 || y_move != 0.0 || z_move != 0.0 || z_move_scale != 0.0 {
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((_, camera_transform)) = active_camera.entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {   
                let mut camera_join = (&cameras, &transforms).join();
                let mut feched_map = map;
                let chunks = feched_map.get_all_chunks()
                for chunk in {

                }
            }
    }
}