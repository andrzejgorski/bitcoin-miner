use crate::pause::PauseMenuState;

use amethyst::{
    audio::output::init_output,
    core::Time,
    ecs::prelude::{Entity, WorldExt},
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    winit::VirtualKeyCode,
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
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        // needed for registering audio output.
        init_output(&mut world);

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/example.ron", ())));
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
                .delete_entity(root_entity)
                .expect("Failed to remove Game Screen");
        }

        self.ui_root = None;
        self.fps_display = None;
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
