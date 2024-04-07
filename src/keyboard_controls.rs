

use crate::characters::{Controller, character_statistics::CharStatistic};
use crate::sdl_thing::window::State;

#[derive(Debug, PartialEq,Clone, Copy, Eq, Hash)]
pub enum Message {
    PlayerControl(Controller),
    WindowControl(State),
    EditStat(CharStatistic)
}



