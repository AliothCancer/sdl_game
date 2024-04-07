use arrayvec::ArrayVec;

use crate::keyboard_controls::Message;
use crate::sdl_thing::event_handler::MessageExecutor;
use crate::FPS;

use super::character_statistics::{CharStatistic, StatEdit};
use super::Character;



pub fn create(name: &str) -> Character {
    Character::default_player(name.to_string())
}

impl MessageExecutor<ArrayVec<Message,5>> for Character {
    fn execute(&mut self, messages: ArrayVec<Message, 5>) {
        messages.into_iter().for_each(|message| {
            if let Message::PlayerControl(control) = message {
                self.movement(control);
            } else if let Message::EditStat(CharStatistic::Speed(StatEdit::Increase(value))) = message {
                self.increment_speed(FPS as i32 * value as i32)
            } else if let Message::EditStat(CharStatistic::Speed(StatEdit::Decrease(value))) = message {
                self.decrement_speed(FPS as i32 * value as i32)
            }
        })
    }
}
