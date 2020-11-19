use amethyst::{
    core::{math::{Point3}},
    prelude::*,
    tiles::Tile,
 };
// use rand::Rng;

#[derive(Default, Clone)]
pub struct ExampleTile;
impl Tile for ExampleTile {
    fn sprite(&self, _point: Point3<u32>, _: &World) -> Option<usize> {
        //let mut rng = rand::thread_rng();
        //let tile_value = rng.gen_range(0, 255);
        //println!("{}",tile_value);
        Some(40)
    }
}