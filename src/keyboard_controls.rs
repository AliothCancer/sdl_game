use crate::characters::Controller;
use crate::sdl_thing::window::State;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Message {
    PlayerControl(Controller),
    WindowControl(State),
    NotMapped,
}

