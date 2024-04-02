use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

use crate::characters::{Character, Controller};

const TITLE: &str = "WINDOW TITLE";
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub fn get_window() -> SdlWindow {
    let context = sdl2::init().unwrap();

    let video_subsystem = context.video().unwrap();
    let window = video_subsystem
        .window(TITLE, WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();

    SdlWindow::new(canvas, context)
}

pub struct SdlWindow {
    pub canvas: Canvas<Window>,
    pub event: EventPump,
    pub state: State,
    characters: Vec<Character>,
}

impl SdlWindow {
    pub fn add_characters(&mut self, character: Character) {
        self.characters.push(character);
    }
    pub fn draw_characters(&mut self) {
        self.characters.iter().for_each(|character| {
            self.canvas.set_draw_color(character.get_color());
            self.canvas.fill_rect(character.rect).unwrap();
            self.canvas.present();
        });
    }
    pub fn new(canvas: Canvas<Window>, context: Sdl) -> Self {
        let event = context.event_pump().unwrap();
        Self {
            canvas,
            event,
            state: State::Continue,
            characters: vec![],
        }
    }
    pub fn setup(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        self.canvas.present();
    }
    
    fn event_mapper(&mut self) -> impl Iterator<Item = Message>{

        self.event.pump_events();
        
        let kbd_state = self.event.keyboard_state();
        let messages = kbd_state.pressed_scancodes().filter_map(|scancode|{
            let mess = match scancode {
                Scancode::W => Message::PlayerControl(Controller::Up),
                Scancode::A => Message::PlayerControl(Controller::Left),
                Scancode::S => Message::PlayerControl(Controller::Down),
                Scancode::D => Message::PlayerControl(Controller::Right),
                Scancode::Escape => Message::WindowControl(State::Stop),
                _ => Message::NotMapped
            };
            match mess{
                Message::NotMapped => None,
                _ => Some(mess)
            }
        });
        
        //std::thread::sleep(Duration::from_millis(100));
        messages
    }

    pub fn from_message_to_action(&mut self) {
        let message = self.from_event_to_message_v2();

        match message {
            Message::WindowControl(State::Stop) => self.state = State::Stop,
            Message::PlayerControl(control) => self.update_characters(control),
            _ => (),
        }
    }
    fn update_characters(&mut self, control: Controller) {
        self.characters[0].movement(control);
        println!("{:?}", self.characters);
    }
}

#[derive(Debug)]
enum Message {
    PlayerControl(Controller),
    WindowControl(State),
    NotMapped
}

#[derive(Debug)]
pub enum State {
    Continue,
    Stop,
}
