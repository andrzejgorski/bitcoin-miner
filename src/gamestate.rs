use crate::pause::PauseMenuState;
use crate::exampletile::ExampleTile;


use amethyst::{
    core::{Time,math::{Vector3},Parent},
    ecs::prelude::{Entity, WorldExt},
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    winit::VirtualKeyCode,
    tiles::{MortonEncoder, TileMap},
    window::ScreenDimensions,
};
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
         Transform,
    },

    //input::{is_close_requested, is_key_down, InputBundle, InputHandler, StringBindings},
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
    // If the Game is paused or not
    paused: bool,
    // The UI root entity. Deleting this should remove the complete UI
    ui_root: Option<Entity>,
    // A reference to the FPS display, which we want to interact with
    fps_display: Option<Entity>,
    // A reference to the random text, which we want to modify during updates
    //sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        
        //let StateData { mut world, .. } = data;
        let world = data.world;
        // needed for registering audio output.
        //init_output(&mut world);

        let _ = world
            .create_entity()
            .with(DebugLinesComponent::with_capacity(1))
            .build();

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/example.ron", ())));

        let a_sprite_sheet_handle = load_sprite_sheet(world, "texture/cp437_20x20.png", "texture/cp437_20x20.ron");
        //self.sprite_sheet_handle = Some(a_sprite_sheet_handle);

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

        

        log::info!("World Created");

        let map = TileMap::<ExampleTile, MortonEncoder>::new(
            Vector3::new(48, 48, 1),
            Vector3::new(20, 20, 1),
            Some(a_sprite_sheet_handle)
        );

        log::info!("map defined");

        let _map_entity = world
            .create_entity()
            .with(map)
            .with(Transform::default())
            .build();

        let _camera = world
            .create_entity()
            .with(Transform::from(Vector3::new(0.0, 0.0, 1.1)),)
            .with(Parent{entity: _map_entity})
            .with(Camera::standard_2d(width, height),)
            .named("camera")
            .build();

        log::info!("map created");
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = true;
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = false;
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                //.delete_all()
                .delete_entity(root_entity)
                .expect("Failed to remove Game Screen");
        }

        self.ui_root = None;
        self.fps_display = None;
        log::info!("Zamykanie Stanu Gra");
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


    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;
        
        // this cannot happen in 'on_start', as the entity might not be fully
        // initialized/registered/created yet.
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
                    //println!("FPS: {}", fps);
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }
        
        Trans::None
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