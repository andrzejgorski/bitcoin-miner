use amethyst::{
    ecs::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    shrev::EventChannel,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
    TransEvent,
};
use crate::game_data::CustomGameData;
use crate::states::MainMenuState;

/// Adapted, originally from amethyst/evoli src/states/pause_menu.rs
// ID's for buttons in the prefab. Required to identify them.

const BUTTON_RESUME_ID: &str = "resume";
const BUTTON_SAVE_ID: &str = "save_game";
const BUTTON_OPTIONS_ID: &str = "paused_options";
const BUTTON_EXIT_TO_MAIN_MENU_ID: &str = "exit_to_main_menu";
const BUTTON_EXIT_ID: &str = "exit";

#[derive(Default)]
pub struct PauseMenuState {
    // button entities are created in on_start() and destroyed in on_stop()
    button_resume: Option<Entity>,
    button_save: Option<Entity>,
    button_options: Option<Entity>,
    button_exit_to_main_menu: Option<Entity>,
    button_exit: Option<Entity>,
    root: Option<Entity>,
}

// load the pause_menu.ron prefab then instantiate it
// if the "resume" button is clicked, goto MainGameState
// if the "exit_to_main_menu" button is clicked, remove the pause and main game states and go to MenuState.
// if the "exit" button is clicked, quit the program.
impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for PauseMenuState {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let world = data.world;

        self.root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/pause_menu.ron", ())));
    }

    fn on_stop(&mut self, data: StateData<CustomGameData>) {
        if let Some(root) = self.root {
            if data.world.delete_entity(root).is_ok() {
                self.root = None;
            }
        }
        self.button_resume = None;
        self.button_save = None;
        self.button_options = None;
        self.button_exit_to_main_menu = None;
        self.button_exit = None;
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
                    log::info!("[Trans::Pop] Closing Pause Menu!");
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_resume {
                    log::info!("Resuming Game!");
                    Trans::Pop
                } else if Some(target) == self.button_exit_to_main_menu {
                    let mut state_transition_event_channel = data
                        .world
                        .write_resource::<EventChannel<TransEvent<CustomGameData, StateEvent>>>();

                    // this allows us to first 'Pop' this state, and then exchange whatever was
                    // below that with a new MainMenu state.
                    state_transition_event_channel.single_write(Box::new(|| Trans::Pop));
                    state_transition_event_channel
                        .single_write(Box::new(|| Trans::Switch(Box::new(MainMenuState::default()))));

                    log::info!("[Trans::Pop] Closing Pause Menu!");
                    log::info!("[Trans::Switch] Switching to MainMenu!");
                    
                    Trans::None// we could also not add the pop to the channel and Pop here
                                // but like this the execution order is guaranteed (in the next versions)
                } else if Some(target) == self.button_exit {
                    Trans::Quit
                } else {
                    Trans::None
                }
                
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::ClickStart,
                target,
            }) => {
                
                if Some(target) == self.button_resume || 
                    Some(target) ==  self.button_save ||
                    Some(target) ==  self.button_exit_to_main_menu ||
                    Some(target) ==  self.button_options ||
                    Some(target) ==  self.button_exit {
                    crate::move_button_on_click::move_button(data, Some(target), 5.)
                }
                Trans::None
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::ClickStop,
                target,
            }) => {
                if Some(target) == self.button_resume || 
                    Some(target) ==  self.button_save ||
                    Some(target) ==  self.button_exit_to_main_menu ||
                    Some(target) ==  self.button_options ||
                    Some(target) ==  self.button_exit {
                    crate::move_button_on_click::move_button(data, Some(target), -5.)
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent>  {
        data.data.update(&data.world, false);
        // once deferred creation of the root ui entity finishes, look up buttons
        if self.button_resume.is_none()
            || self.button_exit_to_main_menu.is_none()
            || self.button_exit.is_none()
            || self.button_options.is_none()
        {
            data.world.exec(|ui_finder: UiFinder<'_>| {
                self.button_resume = ui_finder.find(BUTTON_RESUME_ID);
                self.button_save = ui_finder.find(BUTTON_SAVE_ID);
                self.button_exit_to_main_menu = ui_finder.find(BUTTON_EXIT_TO_MAIN_MENU_ID);
                self.button_options = ui_finder.find(BUTTON_OPTIONS_ID);
                self.button_exit = ui_finder.find(BUTTON_EXIT_ID);
            });
        }
        Trans::None
    }
}
