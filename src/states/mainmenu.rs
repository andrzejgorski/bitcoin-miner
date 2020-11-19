
use amethyst::{
    ecs::{Entity, WorldExt},
    ui::UiEventType,
    prelude::StateData,
    StateEvent,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiEvent, UiFinder},
    winit::VirtualKeyCode,
};

use crate::states::GameState;

use crate::game_data::CustomGameData;

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

impl  <'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for MainMenuState {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        
        self.ui_root =
                Some(data.world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
        
        log::info!("Ui Loaded");
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = data;
        if self.button_new.is_none() || self.button_load.is_none() ||
            self.button_options.is_none() || self.button_options.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_new = ui_finder.find(BUTTON_NEW_ID);
                self.button_load = ui_finder.find(BUTTON_LOAD_ID);
                self.button_options = ui_finder.find(BUTTON_OPTIONS_ID);
                self.button_quit = ui_finder.find(BUTTON_QUIT_ID); 
                //log::info!("test button: {:?}, fps: {:?}", ui_finder.find("test"), ui_finder.find("fps"));
                
            });
        } 
        
        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
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
    fn on_stop(&mut self, data: StateData<CustomGameData>) {
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
}/*
fn move_button (data: StateData<CustomGameData>,entity: Option<Entity>, y_trasnaltion: f32){
    let StateData { world, .. } = data;
    let mut ui_transform = world.write_storage::<UiTransform>();
    if let Some(entity) = entity.and_then(|entity| ui_transform.get_mut(entity)) {
        entity.local_y = entity.local_y - y_trasnaltion;
    }
}*/