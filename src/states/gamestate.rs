use crate::states::PauseMenuState;
use crate::exampletile::ExampleTile;

use amethyst::{
    core::{Time,math::Vector3,Transform},
    ecs::prelude::{Entity, WorldExt, Component, DenseVecStorage},
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    winit::VirtualKeyCode,
    tiles::{MortonEncoder, TileMap},
    window::ScreenDimensions,
    assets::{AssetStorage, Loader, Handle},
    renderer::{
        camera::{ Camera, ActiveCamera},
        debug_drawing::DebugLinesComponent,
        formats::texture::ImageFormat,
        sprite::{ SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        Texture,
    },
};

#[derive(Default)]
pub struct GameState {
    paused: bool,
    ui_root: Option<Entity>,
    fps_display: Option<Entity>,
    map: Map,
}

pub const CHUNK_SIZE: u32 = 32;
pub const TEXTURE_SIZE: u32 = 32;

#[derive(Default)]
pub struct Map {
    pub chunks: Vec<Chunk>,
    
}

pub struct Chunk {
    pub tiles: TileMap::<ExampleTile, MortonEncoder>,
    pub x:i32,
    pub y:i32,
}
impl Map {
    fn replace(map: Map) -> Map {
        map
    }
}
impl Chunk {
    fn new(tiles: TileMap::<ExampleTile, MortonEncoder>, x: i32, y: i32) -> Chunk {
        Chunk {
            tiles: tiles,
            x: x,
            y: y,
        }
    }
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}
impl Component for Chunk {
    type Storage = DenseVecStorage<Self>;
}
fn init_new_tiles(handle: Handle<SpriteSheet>) -> TileMap::<ExampleTile, MortonEncoder> {
    let new_tiles = TileMap::<ExampleTile, MortonEncoder>::new(
        Vector3::new(CHUNK_SIZE, CHUNK_SIZE, 1),
        Vector3::new(TEXTURE_SIZE, TEXTURE_SIZE, 1),
        Some(handle)
    );
    return new_tiles;
}

fn new_chunk(x:i32,y:i32, tiles: TileMap::<ExampleTile, MortonEncoder>, world: &mut World){
    {
        let chunk = Chunk::new(tiles.clone(), 0, 0);
        //let world_clone = world.clone();
        let map = world.try_fetch_mut::<Map>();
        
        if let Some(mut fetched_map) = map {

        fetched_map.chunks.push(chunk);

        } else {
        println!("No Map present in `World`");
        }
    }
    {
        let _map_entity = world
                .create_entity()
                .named(format!("Chunk_{:?}_{:?}", x, y))
                .with(tiles)
                .with(Transform::from(Vector3::new((x*1024) as f32, (y*1024) as f32, 0.)),)
                .build();
    }
}
//fn init_new_Chunk(tiles: TileMap::<ExampleTile, MortonEncoder>, x:i32, y:i32 ) -> Chunk {
//
//}

impl SimpleState for GameState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;
        // needed for registering audio output.
        //init_output(&mut world);

        let _ = world
            .create_entity()
            .with(DebugLinesComponent::with_capacity(1))
            .build();

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/example.ron", ())));

        let sprite_sheet_handle = load_sprite_sheet(world, "texture/nature_tileset.png", "texture/nature_tileset.ron");
        //self.sprite_sheet_handle = Some(sprite_sheet_handle);
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let _camera = world
            .create_entity()
            .with(Transform::from(Vector3::new(0.0, 0.0, 1.1)),)
            .with(Camera::standard_2d(width, height),)
            .named("camera")
            .build();

        self.map = Map::default();
        let tiles = init_new_tiles(sprite_sheet_handle);

        new_chunk(0,0, tiles.clone(), world);
        new_chunk(1,0, tiles.clone(), world);
        new_chunk(2,0, tiles.clone(), world);

        let _camera = world
            .create_entity()
            .with(Transform::from(Vector3::new(0.0, 0.0, 1.1)),)
            .with(Camera::standard_2d(width, height),)
            .named("camera")
            .build();

    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        //data.data.update(&data.world, false);
        let StateData { world, .. } = data;
       
        if self.fps_display.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("fps") {
                    self.fps_display = Some(entity);
                }
            });
        }
        
        // it is important that the 'paused' field is actually pausing your game.
        // Make sure to also pause your running systems.
        
        if !self.paused {
            let mut ui_text = world.write_storage::<UiText>();
            
            if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
                if world.read_resource::<Time>().frame_number() % 20 == 0 && !self.paused {
                    let fps = world.read_resource::<FpsCounter>().sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }
        
        Trans::None
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = true;
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = false;
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Push] Pausing Game!");
                    Trans::Push(Box::new(PauseMenuState::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(_ui_event) => {
                /*log::info!(
                    "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                    ui_event
                );*/
                Trans::None
            }
            StateEvent::Input(_input) => {
                //log::info!("Input Event detected: {:?}.", input);
                Trans::None
            }
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove Game Screen");
        }

        self.ui_root = None;
        self.fps_display = None;
        log::info!("Zamykanie Stanu Gra");
    }
}


fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(png_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}