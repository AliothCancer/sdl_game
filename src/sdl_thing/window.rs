use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::{characters::Character, keyboard_controls::Message};

use super::event_handler::{maps_events_to_messages, MessageExecutor};

const TITLE: &str = "WINDOW TITLE";
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum State {
    Continue,
    Stop,
}

pub struct SdlWindow {
    pub canvas: Canvas<Window>,
    pub event: EventPump,
    pub state: State,
    characters: Vec<Character>,
}

impl SdlWindow {
    // characters interface
    pub fn add_characters(&mut self, character: Character) {
        self.characters.push(character);
    }
    pub fn update(&mut self){
        let messages = maps_events_to_messages(&mut self.event);
        self.execute(&messages);
        self.update_characters(messages);
    }
    fn update_characters(&mut self, messages: HashSet<Message>) {
        self.characters[0].execute(&messages);
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
            .window(TITLE, WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

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
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }
}


impl MessageExecutor for SdlWindow{
    fn execute(&mut self, messages: &HashSet<Message>) {
        messages.iter().for_each(|message| {
            if let Message::WindowControl(state) = message {
                self.state = *state;
            }
        })
    }
}