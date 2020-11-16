use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{System, SystemData},
    ecs::{Read, ReadStorage, WriteStorage},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiText, UiTransform}
};

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
        ReadStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (events, _transforms, mut texts): Self::SystemData) {
        // Reader id was just initialized above if empty
        for event in events.read(&mut self.reader_id) {
            let button_text = texts.get_mut(event.target).unwrap();
    
            match event.event_type {
                UiEventType::HoverStart => { 
                    button_text.color[3] = 1.0; 
                },
                UiEventType::HoverStop  => { 
                    button_text.color[3] = 0.5; 
                },
                _ => {},
            }   
        }
    }
}