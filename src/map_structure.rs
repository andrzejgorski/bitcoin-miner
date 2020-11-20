use amethyst::{
    assets::Handle,
    core::{math::Point3,math::Vector3,Transform},
    ecs::prelude::{World, WorldExt,Entity, Component, DenseVecStorage},
    prelude::*,
    tiles::{Tile, MortonEncoder, TileMap},
    renderer::sprite::SpriteSheet,
};

pub const CHUNK_SIZE: u32 = 32;
pub const TEXTURE_SIZE: u32 = 32;

#[derive(Default, Clone)]
pub struct ExampleTile;
impl Tile for ExampleTile {
    fn sprite(&self, _point: Point3<u32>, _: &World) -> Option<usize> {
        Some(40)
    }
}

#[derive( Clone)]
pub struct Chunk {
    pub tiles: TileMap::<ExampleTile, MortonEncoder>,
    pub x:i32,
    pub y:i32,
    pub entity: Option<Entity>,
}

impl Chunk {
    fn new(tiles: TileMap::<ExampleTile, MortonEncoder>, x: i32, y: i32) -> Self {
        Chunk {
            tiles: tiles,
            x: x,
            y: y,
            entity: None,
        }
    }
}

impl Component for Chunk {
    type Storage = DenseVecStorage<Self>;
}
#[derive(Clone)]
pub struct Map {
    pub chunks: Vec<Chunk>,
    pub seed: i32,
    pub sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl Map {

    pub fn new(handle: Option<Handle<SpriteSheet>>, seed: i32) -> Self {
        Map {
            chunks: vec![],
            seed: seed,
            sprite_sheet_handle: handle,
        }
    }

    pub fn get_chunk(&mut self, x: i32, y: i32) -> Chunk {

        let chunks = self.chunks.clone();
        for chunk in chunks{
            if chunk.x == x && chunk.y == y {
                return chunk;
            }
        }
        self.generate_new_chunk(x, y);
        //eprintln!("Note: Chunk not found");
        return self.get_chunk(x,y);
    }

    pub fn generate_tiles(&mut self, x:i32, y:i32 ) -> TileMap::<ExampleTile, MortonEncoder> {
        let new_tiles = TileMap::<ExampleTile, MortonEncoder>::new(
            Vector3::new(CHUNK_SIZE, CHUNK_SIZE, 1),
            Vector3::new(TEXTURE_SIZE, TEXTURE_SIZE, 1),
            self.clone().sprite_sheet_handle
        );
        return new_tiles;
    }
    
    pub fn generate_new_chunk(&mut self, x:i32,y:i32){
        let tiles = self.generate_tiles(x * CHUNK_SIZE as i32, y * CHUNK_SIZE as i32);
        let chunk = Chunk::new(tiles.clone(), x, y);
        self.chunks.push(chunk);
    }

    pub fn create_chunk_entity(&mut self, x:i32,y:i32, world: &mut World){

        let mut chunk = self.get_chunk(x, y);
        let tiles = chunk.tiles;

        let _tileset_entity = world
            .create_entity()
            .named(format!("Chunk_{:?}_{:?}", x, y))
            .with(tiles)
            .with(Transform::from(Vector3::new((x*1024) as f32, (y*1024) as f32, 0.)),)
            .build();
        chunk.entity = Some(_tileset_entity);
    }
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}