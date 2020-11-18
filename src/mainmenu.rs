use amethyst::ecs::{Entity, WorldExt};
use amethyst::ui::UiEventType;
use amethyst::prelude::{ GameData, SimpleState, SimpleTrans, StateData};
use amethyst::StateEvent;
use amethyst::{
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiEvent, UiFinder},
    winit::VirtualKeyCode,
};

use crate::gamestate::GameState;

pub const BUTTON_NEW: &str = "new game";
pub const BUTTON_LOAD: &str = "load game";
pub const BUTTON_OPTIONS: &str = "options";
pub const BUTTON_QUIT: &str = "quit";

#[derive(Default)]
pub struct MainMenuState {
    ui_root: Option<Entity>,
    button_new: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_quit: Option<Entity>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("Start Stanu Menu");
        self.ui_root =
                Some(data.world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;
        if self.button_new.is_none() || self.button_load.is_none() ||
            self.button_options.is_none() || self.button_options.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_new = ui_finder.find(BUTTON_NEW);
                self.button_load = ui_finder.find(BUTTON_LOAD);
                self.button_options = ui_finder.find(BUTTON_OPTIONS);
                self.button_quit = ui_finder.find(BUTTON_QUIT); 
                log::info!("test button: {:?}, fps: {:?}", ui_finder.find("test"), ui_finder.find("fps"));
                
            });
        }


        Trans::None
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Switch] Switching back to WelcomeScreen!");
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_new {
                    log::info!("[Trans::Switch] Switching to Game!");
                    return Trans::Switch(Box::new(GameState::default()));
                }
                if Some(target) == self.button_load || Some(target) == self.button_options {
                    log::info!("This Buttons functionality is not yet implemented!");
                }
                if Some(target) == self.button_quit || Some(target) == self.button_quit {
                    log::info!("Bye Bye!");
                    return Trans::Quit;
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }
    fn on_stop(&mut self, data: StateData<GameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }
        
        self.ui_root = None;
        self.button_new = None;
        self.button_load = None;
        self.button_options = None;
        self.button_quit = None;
    }
}