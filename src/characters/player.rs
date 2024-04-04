use arrayvec::ArrayVec;

use crate::keyboard_controls::Message;
use crate::sdl_thing::event_handler::MessageExecutor;

use super::Character;

pub fn create(name: &str) -> Character {
    Character::default_player(name.to_string())
}

impl MessageExecutor for Character {
    fn execute(&mut self, messages: ArrayVec<Message, 5>) {
        messages.into_iter().for_each(|message| {
            if let Message::PlayerControl(control) = message {
                self.movement(control)
            }
        })
    }
}
