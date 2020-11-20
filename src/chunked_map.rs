use std::collections::HashMap;

pub const CHUNK_SIZE: usize = 32;

enum Terrain {
    Grass,
    Water,
}

impl Default for Terrain {
    fn default() -> Terrain { Terrain::Grass }
}

#[derive(Default)]
pub struct BTCMTile {
    terrain: Terrain,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ChunkCoordinates {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct TileMapCoordinates {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct TileChunkCoordinates {
    x: usize,
    y: usize,
}

impl TileMapCoordinates {
    fn div(&self) -> (ChunkCoordinates, TileChunkCoordinates) {
        (ChunkCoordinates{
            x: self.x / CHUNK_SIZE as i32,
            y: self.y / CHUNK_SIZE as i32,
        }, TileChunkCoordinates {
            x: (self.x % CHUNK_SIZE as i32) as usize,
            y: (self.y % CHUNK_SIZE as i32) as usize,
        })
    }
}

pub struct Chunk <T: Default> {
    pub tiles: [[T; CHUNK_SIZE]; CHUNK_SIZE],
    pub coordinates: ChunkCoordinates,
}


impl <T: Default + Copy> Chunk <T> {
    fn new(coordinates: ChunkCoordinates) -> Self {
        let tiles = [[T::default(); CHUNK_SIZE]; CHUNK_SIZE];
        Chunk {
            tiles: tiles,
            coordinates
        }
    }

    fn  get_tile<'a, 'b> (self: &'a mut Self, coordinates: &'b TileChunkCoordinates) -> &'a mut T {
        &mut self.tiles[coordinates.x][coordinates.y]
    }
}

pub struct ChunkedMap <T: Default> {
    pub chunks: HashMap<ChunkCoordinates, Chunk<T>>,
    seed: i32,
}

impl <T: Default + Copy> ChunkedMap <T> {

    pub fn new(seed: i32) -> Self {
        ChunkedMap {
            chunks: HashMap::default(),
            seed
        }
    }

    pub fn get_tile(&mut self, coordinates: &TileMapCoordinates) -> &mut T {
        let (chunkCoordinates, tileCoordinates) = coordinates.div();
        self.get_chunk(chunkCoordinates).get_tile(&tileCoordinates)
    }

    pub fn get_chunk(&mut self, coordinates: ChunkCoordinates) -> &mut Chunk<T> {
        self.chunks.entry(coordinates).or_insert(Chunk::new(coordinates))
    }
}
