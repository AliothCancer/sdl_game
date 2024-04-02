pub mod player;

use crate::sdl_thing::window::{HEIGHT, WIDTH};
use sdl2::{pixels::Color, rect::Rect};

#[allow(unused)]
#[derive(Debug)]
pub struct Character {
    name: String,
    hp: u8,
    pub rect: Rect,
    color: Color,
    speed: i32,
}

#[derive(Debug, PartialEq, Eq, Hash,Clone, Copy)]
pub enum Controller {
    Up,
    UpLeft,
    UpRight,

    Down,
    DownLeft,
    DownRight,

    Left,
    Right,
}

impl Character {
    pub fn movement(&mut self, control: Controller) {
        match control {
            Controller::Down => self.rect.set_y(self.rect.y + self.speed),
            Controller::Up => self.rect.set_y(self.rect.y - self.speed),
            Controller::Left => self.rect.set_x(self.rect.x - self.speed),
            Controller::Right => self.rect.set_x(self.rect.x + self.speed),
            Controller::DownLeft => {
                self.rect.set_y(self.rect.y + self.speed);
                self.rect.set_x(self.rect.x - self.speed);
            }
            Controller::DownRight => {
                self.rect.set_y(self.rect.y + self.speed);
                self.rect.set_x(self.rect.x + self.speed);
            }
            Controller::UpLeft => {
                self.rect.set_y(self.rect.y - self.speed);
                self.rect.set_x(self.rect.x - self.speed)
            }
            Controller::UpRight => {
                self.rect.set_y(self.rect.y - self.speed);
                self.rect.set_x(self.rect.x + self.speed);
            }
        }
    }
    pub fn default_player(name: String) -> Self {
        let coor = ((WIDTH / 2) as i32, (HEIGHT / 2) as i32);
        Character {
            name,
            hp: 100,
            rect: Rect::new(coor.0, coor.1, 100, 100), // Default rect dimensions
            color: Color::BLUE,                        // Default color
            speed: 10,
        }
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }
    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }
}
