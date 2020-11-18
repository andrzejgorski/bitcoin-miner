use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{System, SystemData},
    ecs::{Read, ReadStorage,  WriteStorage, Join},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiTransform, UiText},
};
use crate::mainmenu::{BUTTON_NEW, BUTTON_LOAD, BUTTON_OPTIONS, BUTTON_QUIT};

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
        WriteStorage<'s, UiText>,
        WriteStorage<'s, UiTransform>,
    );

    fn run(&mut self, (events, uiText, mut transforms): Self::SystemData) {

        //let evs = events.read(&mut self.reader_id);
        
        for event in events.read(&mut self.reader_id) {
            //println!("event target: {:?}" , transforms.get_mut(event.target).unwrap().id);
            
            //for (trans, texts) in (&mut transforms, &uiText).join() {
            
                //let element = transforms.get_mut(event.target).unwrap(); 

                    
                
                //let button_text = texts.get_mut(event.target).unwrap();
                //let trans = transforms.get_mut(event.target).unwrap();
                    
                /*
                if trans.id == BUTTON_NEW || trans.id == BUTTON_LOAD ||
                    trans.id == BUTTON_OPTIONS || trans.id == BUTTON_QUIT {
                    match event.event_type {
                        UiEventType::ClickStart  => { 
                            
                            trans.local_y -= 2.0;
                        },
                        UiEventType::ClickStop  => { 
                            trans.local_y += 2.0;
                        },
                        _ => {},
                    }
                }*/
                    
            
        }
        
    }
}