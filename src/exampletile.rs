use amethyst::{
    core::{math::{Point3}},
    prelude::*,
    tiles::Tile,
 };
 
#[derive(Default, Clone)]
pub struct ExampleTile;
impl Tile for ExampleTile {
    fn sprite(&self, _point: Point3<u32>, _: &World) -> Option<usize> {
        Some(40)
    }
}
