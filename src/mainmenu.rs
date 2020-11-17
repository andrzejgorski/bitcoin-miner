
use amethyst::assets::{Loader};
use amethyst::ecs::{Entity, World, WorldExt};
use amethyst::ui::{
    Anchor, LineMode, UiText, UiTransform, UiEventType, Interactable,TtfFormat,
};
use amethyst::prelude::{Builder, GameData, SimpleState, SimpleTrans, StateData};
use amethyst::StateEvent;

pub const TEMP_LOGO: &str = "Bitcoin miner";

pub const BUTTON_NEW: &str = "new game";
pub const BUTTON_LOAD: &str = "load game";
pub const BUTTON_OPTIONS: &str = "options";
pub const BUTTON_QUIT: &str = "quit";

#[derive(Default)]
pub struct MainMenuState {
    game_logo: Option<Entity>,
    button_new: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_quit: Option<Entity>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.game_logo = initialise_ui_element(-3, TEMP_LOGO, world);

        self.button_new = initialise_ui_element(0, BUTTON_NEW, world);
        self.button_load = initialise_ui_element(1, BUTTON_LOAD, world);
        self.button_options = initialise_ui_element(2, BUTTON_OPTIONS,  world);
        self.button_quit = initialise_ui_element(3, BUTTON_QUIT, world);
    }

    fn handle_event(
        &mut self,
    	_data: StateData<'_, GameData<'_, '_>>,
    	event: StateEvent) -> SimpleTrans {
    	if let StateEvent::Ui(ui_event) = event {
    		let is_target = ui_event.target == self.button_quit.unwrap();

    		match ui_event.event_type {
    			UiEventType::Click if is_target => {
                    return SimpleTrans::Quit;
                    
    			},
    			_ => {
    				return SimpleTrans::None;
    			},  
    		};
    	}

    	SimpleTrans::None
    }
}

fn initialise_ui_element(position: i32, label: &str, world: &mut World) -> Option<Entity> {

    let color = if label == BUTTON_QUIT 
    {
        [1.0f32, 0.3f32, 0.3f32, 0.5f32]
    }   else {
        [1.0f32, 1.0f32, 1.0f32, 0.5f32]
    };

    let font_size = if label == TEMP_LOGO{ 30.0 } else { 25.0 }; 

    let font_handle = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
        );

    /* Create the transform */
    let ui_transform = UiTransform::new(
        String::from(label), // id
        Anchor::Middle,                // anchor
        Anchor::Middle,                // pivot
        0f32,                          // x
        ((position * -30) ) as f32,   // y
        0f32,                          // z
        400f32,                        // width
        35f32,                         // height
        );
    /* Create the text */
    let ui_text = UiText::new(
        font_handle,                      // font
        String::from(label),                    // text
        color, // color
        font_size,                            // font_size
        LineMode::Single,                 // line_mode
        Anchor::Middle,                   // align
        );

    /* Building the entity */
    let ui = world.create_entity()
        .with(ui_transform)
        .with(ui_text)
        .with(Interactable)   
        .build();
 
    return Some(ui);
}