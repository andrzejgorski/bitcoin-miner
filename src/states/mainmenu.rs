
use amethyst::{
    ecs::{Entity, WorldExt},
    ui::UiEventType,
    prelude::*,
    StateEvent,
    input::{is_close_requested, is_key_down},
    ui::{UiCreator, UiEvent, UiFinder},
    winit::VirtualKeyCode,
};
use crate::states::GameState;

pub const BUTTON_NEW_ID: &str = "new game";
pub const BUTTON_LOAD_ID: &str = "load game";
pub const BUTTON_OPTIONS_ID: &str = "options";
pub const BUTTON_QUIT_ID: &str = "exit";

#[derive(Default)]
pub struct MainMenuState {
    ui_root: Option<Entity>,
    button_new: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_quit: Option<Entity>,
}

impl  SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        
        self.ui_root =
                Some(data.world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
        
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        //data.data.update(&data.world, false);
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;
        if self.button_new.is_none() || self.button_load.is_none() ||
            self.button_options.is_none() || self.button_options.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_new = ui_finder.find(BUTTON_NEW_ID);
                self.button_load = ui_finder.find(BUTTON_LOAD_ID);
                self.button_options = ui_finder.find(BUTTON_OPTIONS_ID);
                self.button_quit = ui_finder.find(BUTTON_QUIT_ID); 
            });
        } 
    
        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
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
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::ClickStart,
                target,
            }) => {
                
                if Some(target) == self.button_new || 
                    Some(target) ==  self.button_load ||
                    Some(target) ==  self.button_options ||
                    Some(target) ==  self.button_quit {
                    crate::move_button_on_click::move_button(data, Some(target), 5.)
                }
                Trans::None
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::ClickStop,
                target,
            }) => {
                if Some(target) == self.button_new || 
                    Some(target) ==  self.button_load ||
                    Some(target) ==  self.button_options ||
                    Some(target) ==  self.button_quit {
                    crate::move_button_on_click::move_button(data, Some(target), -5.)
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