use arrayvec::ArrayVec;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::{characters::Character, keyboard_controls::Message};

use super::event_handler::{EventHandler, MessageExecutor};

const TITLE: &str = "WINDOW TITLE";
pub const WINDOW_WIDTH: u32 = 1000;
pub const WINDOW_HEIGHT: u32 = 900;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum State {
    Continue,
    Stop,
}

pub struct SdlWindow {
    pub canvas: Canvas<Window>,
    event_handler: EventHandler,
    pub state: State,
    characters: Vec<Character>,
}

impl SdlWindow {
    // characters interface
    pub fn add_characters(&mut self, character: Character) {
        self.characters.push(character);
    }
    pub fn update(&mut self) {
        self.event_handler.get_messages();
        self.execute(self.event_handler.window_message);
        self.characters[0].execute(self.event_handler.messages.clone());
    }
    pub fn draw_characters(&mut self) {
        self.characters.iter().for_each(|character| {
            self.canvas.set_draw_color(character.get_color());
            self.canvas.fill_rect(character.rect).unwrap();
            self.canvas.present();
        });
    }

    pub fn new() -> Self {
        let context = sdl2::init().unwrap();

        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        let event_handler = EventHandler {
            event: context.event_pump().unwrap(),
            messages: ArrayVec::<Message,5>::new(),
            window_message: Message::WindowControl(State::Continue)
        };
        Self {
            canvas,
            event_handler,
            state: State::Continue,
            characters: vec![],
        }
    }
    pub fn setup(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        self.canvas.present();
    }
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }
}

impl Default for SdlWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageExecutor<Message> for SdlWindow {
    fn execute(&mut self, message: Message) {
        if let Message::WindowControl(state) = message {
                self.state = state;
            }
    }
}
