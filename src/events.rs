use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{System, SystemData},
    ecs::{Read, WriteStorage},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiText, UiTransform,}
};
use crate::mainmenu::TEMP_LOGO;
/// This shows how to handle UI events. This is the same as in the 'ui' example.
#[derive(SystemDesc)]
#[system_desc(name(UiEventHandlerSystemDesc))]
pub struct UiEventHandlerSystem {
    #[system_desc(event_channel_reader)]
    reader_id: ReaderId<UiEvent>,
    
}

impl UiEventHandlerSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'s> System<'s> for UiEventHandlerSystem {
    type SystemData = (
        Read<'s, EventChannel<UiEvent>>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (events, mut transforms, mut texts): Self::SystemData) {
        // Reader id was just initialized above if empty
        for event in events.read(&mut self.reader_id) {
            let button_text = texts.get_mut(event.target).unwrap();
            let trans = transforms.get_mut(event.target).unwrap();
            if trans.id != TEMP_LOGO {
                match event.event_type {
                    UiEventType::HoverStart => { 
                        button_text.color[3] = 1.0; 
                    },
                    UiEventType::HoverStop  => { 
                        button_text.color[3] = 0.5; 
                    },
                    UiEventType::ClickStart  => { 
                        button_text.color[3] = 0.1; 
                        trans.local_y -= 2.0;
                    },
                    UiEventType::ClickStop  => { 
                        button_text.color[3] = 1.0; 
                        trans.local_y += 2.0;
                    },
                    _ => {},
                }
            }
        }
        
    }
}