use sdl2::{keyboard::Scancode, EventPump};

use crate::characters::Controller;
use crate::keyboard_controls::Message;
use crate::sdl_thing::window::State;

pub struct EventHandler {
    pub event: EventPump,
    pub messages: [Message; 5],
}

impl EventHandler {
    pub fn execute_messages() {}
    pub fn get_messages(&mut self) {
        self.event.pump_events();

        let kbd_state = self.event.keyboard_state();
        let messages = kbd_state.pressed_scancodes().filter_map(|scancode| {
            let mess = match scancode {
                Scancode::W => Message::PlayerControl(Controller::Up),
                Scancode::A => Message::PlayerControl(Controller::Left),
                Scancode::S => Message::PlayerControl(Controller::Down),
                Scancode::D => Message::PlayerControl(Controller::Right),
                Scancode::Escape => Message::WindowControl(State::Stop),
                _ => Message::NotMapped,
            };
            match mess {
                Message::NotMapped => None,
                _ => Some(mess),
            }
        });
        
        for (n, message) in messages.enumerate(){
            self.messages[n] = message  
        }
        println!("{:?}", self.messages);
    }
}

// A trait that allow the struct for which is impl to be updated by an HashSet<Message>
#[allow(unused)]
pub trait MessageExecutor {
    fn execute(&mut self, messages: [Message; 5]) {}
}
