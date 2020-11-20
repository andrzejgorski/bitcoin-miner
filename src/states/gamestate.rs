use crate::states::PauseMenuState;
use crate::map_structure::Map;

use amethyst::{
    core::{Time,math::Vector3,Transform},
    ecs::prelude::{Entity, WorldExt},
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    winit::VirtualKeyCode,
    window::ScreenDimensions,
    assets::{AssetStorage, Loader},
    renderer::{
        camera::{ Camera},
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

        let sprite_sheet_handle =  Some(load_sprite_sheet(world, "texture/nature_tileset.png", "texture/nature_tileset.ron"));

        let mut map = Map::new(sprite_sheet_handle, 0);
        map.generate_new_chunk(0,0);
        map.generate_new_chunk(1,0);
        map.generate_new_chunk(2,0);
        map.create_chunk_entity(0, 0, world);
        map.create_chunk_entity(1, 0, world);
        map.create_chunk_entity(2, 0, world);

        world.insert(map);


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