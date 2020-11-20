use amethyst::{
    core::{
        math::Vector3,
        Time, Transform, 
    },
    ecs::{Read, ReadStorage, System, WriteStorage, Join},
    input::{InputHandler, StringBindings},
    tiles::{MortonEncoder, TileMap},
};
use crate::map_structure::*;


pub struct MapMovementSystem {
    rotate: bool,
    translate: bool,
    vector: Vector3<f32>,
}
impl Default for MapMovementSystem {
    fn default() -> Self {
        Self {
            rotate: false,
            translate: false,
            vector: Vector3::new(100.0, 0.0, 0.0),
        }
    }
}
impl<'s> System<'s> for MapMovementSystem {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, TileMap<ExampleTile, MortonEncoder>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (time, mut transforms, tilemaps, input): Self::SystemData) {
        if input.action_is_down("toggle_rotation").unwrap() {
            self.rotate ^= true;
        }
        if input.action_is_down("toggle_translation").unwrap() {
            self.translate ^= true;
        }
        if self.rotate {
            for (_, transform) in (&tilemaps, &mut transforms).join() {
                transform.rotate_2d(time.delta_seconds());
            }
        }
        if self.translate {
            for (_, transform) in (&tilemaps, &mut transforms).join() {
                transform.prepend_translation(self.vector * time.delta_seconds());
                if transform.translation().x > 500.0 {
                    self.vector = Vector3::new(-100.0, 0.0, 0.0);
                } else if transform.translation().x < -500.0 {
                    self.vector = Vector3::new(100.0, 0.0, 0.0);
                }
            }
        }
    }
}