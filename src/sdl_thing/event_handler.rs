
use arrayvec::ArrayVec;
use sdl2::{keyboard::Scancode, EventPump};

use crate::characters::character_statistics::{CharStatistic, StatEdit};
use crate::characters::Controller;
use crate::keyboard_controls::Message;
use crate::sdl_thing::window::State;

pub struct EventHandler {
    pub event: EventPump,
    pub messages: ArrayVec<Message, 5>,
    pub window_message: Message
}

impl EventHandler {
    pub fn execute_messages() {}
    pub fn get_messages(&mut self) {
        
        self.event.pump_events();

        let mut contains_up = false;
        let mut contains_down = false;
        let mut contains_left = false;
        let mut contains_right = false;

        let kbd_state = self.event.keyboard_state();
        self.messages.clear();
        
        

        let messages = kbd_state
            .pressed_scancodes()
            .filter_map(|scancode| {
                Some(match scancode {
                    Scancode::W => {
                        contains_up = true;
                        Message::PlayerControl(Controller::Up)
                    },
                    Scancode::A => {
                        contains_left = true;
                        Message::PlayerControl(Controller::Left)
                    },
                    Scancode::S => {
                        contains_down = true;
                        Message::PlayerControl(Controller::Down)
                    },
                    Scancode::D => {
                        contains_right = true;
                        Message::PlayerControl(Controller::Right)
                    },
                    // increase speed
                    Scancode::I => Message::EditStat(CharStatistic::Speed(StatEdit::Increase(1))),
                    Scancode::O => Message::EditStat(CharStatistic::Speed(StatEdit::Decrease(1))),
                    _ => return None,
                })
            })
            .take(self.messages.capacity());
        
        self.messages.extend(messages);
        
        if contains_down && contains_left{
            let mess = Message::PlayerControl(Controller::DownLeft);
            self.messages.clear();
            self.messages.push(mess)
        }
        if contains_down && contains_right{
            let mess = Message::PlayerControl(Controller::DownRight);
            self.messages.clear();
            self.messages.push(mess)
        }
        if contains_up && contains_left{
            let mess = Message::PlayerControl(Controller::UpLeft);
            self.messages.clear();
            self.messages.push(mess)
        }
        if contains_up && contains_right{
            let mess = Message::PlayerControl(Controller::UpRight);
            self.messages.clear();
            self.messages.push(mess)
        }
        
        if kbd_state.is_scancode_pressed(Scancode::Escape){
            self.window_message = Message::WindowControl(State::Stop);
        } 
    
        println!("{:?}", self.messages);
    
    }
}

// A trait that allow the struct for which is impl to be updated by an array of Message
#[allow(unused)]
pub trait MessageExecutor<T>{
    
    fn execute(&mut self, messages: T) {}
}
