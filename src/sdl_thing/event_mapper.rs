
use std::collections::HashSet;

use sdl2::{keyboard::Scancode, EventPump};

use crate::sdl_thing::State;


fn event_mapper(event: &mut EventPump) -> HashSet<Message> {
    event.pump_events();

    let kbd_state = event.keyboard_state();
    let messages = kbd_state
        .pressed_scancodes()
        .filter_map(|scancode| {
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
        })
        .collect();

    messages
}