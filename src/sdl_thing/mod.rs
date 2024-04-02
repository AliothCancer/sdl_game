pub mod event_mapper;



use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

use crate::characters::{Character, Controller};

const TITLE: &str = "WINDOW TITLE";
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

#[derive(Debug, PartialEq, Eq, Hash)]
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


impl SdlWindow {
    // characters interface
    pub fn add_characters(&mut self, character: Character) {
        self.characters.push(character);
    }
    fn update_characters(&mut self, control: Controller) {
        self.characters[0].movement(control);
        println!("{:?}", self.characters);
    }
    pub fn draw_characters(&mut self) {
        self.characters.iter().for_each(|character| {
            self.canvas.set_draw_color(character.get_color());
            self.canvas.fill_rect(character.rect).unwrap();
            self.canvas.present();
        });
    }

    fn get_window_controls(&mut self){

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
}

