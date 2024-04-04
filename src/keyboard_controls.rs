

use crate::characters::Controller;
use crate::sdl_thing::window::State;

#[derive(Debug, PartialEq,Clone, Copy, Eq, Hash)]
pub enum Message {
    PlayerControl(Controller),
    WindowControl(State),
}



