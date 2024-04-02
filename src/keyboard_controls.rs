
use crate::sdl_thing::State;
use crate::characters::Controller;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Message {
    PlayerControl(Controller),
    WindowControl(State),
    NotMapped,
}




/*
pub fn messages_mapper(event: &mut EventPump) {
    let messages = event_mapper();
    if messages.contains(&Message::WindowControl(State::Stop)) {
        self.state = State::Stop
    }
    println!("{:?}", messages);
}

*/
