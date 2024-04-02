use crate::keyboard_controls::Message;
use crate::sdl_thing::event_handler::MessageExecutor;
use std::collections::HashSet;

use super::Character;

pub fn create(name: &str) -> Character {
    Character::default_player(name.to_string())
}

impl MessageExecutor for Character {
    fn execute(&mut self, messages: &HashSet<Message>) {
        messages.iter().for_each(|message| {
            if let Message::PlayerControl(control) = message {
                self.movement(*control)
            }
        })
    }
}
