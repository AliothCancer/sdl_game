pub mod player;
pub mod character_statistics;

use crate::{sdl_thing::window::{WINDOW_HEIGHT, WINDOW_WIDTH}, FPS};
use sdl2::{pixels::Color, rect::Rect};

#[allow(unused)]
#[derive(Debug)]
pub struct Character {
    name: String,
    hp: u8,
    pub rect: Rect,
    color: Color,
    speed: i32, // pixels to move per cycle
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
        let diagonal_increment = (self.speed as f32 / (0.5_f32).sqrt()) as i32;
        let ortogonal_increment = self.speed;
        match control {
            Controller::Down => self.rect.set_y(self.rect.y + ortogonal_increment),
            Controller::DownLeft => {
                self.rect.set_y(self.rect.y + diagonal_increment);
                self.rect.set_x(self.rect.x - diagonal_increment);
            },
            Controller::DownRight => {
                self.rect.set_y(self.rect.y + diagonal_increment);
                self.rect.set_x(self.rect.x + diagonal_increment);
            },

            Controller::Up => self.rect.set_y(self.rect.y - ortogonal_increment),
            Controller::UpLeft => {
                self.rect.set_y(self.rect.y - diagonal_increment);
                self.rect.set_x(self.rect.x - diagonal_increment);
            },
            Controller::UpRight => {
                self.rect.set_y(self.rect.y - diagonal_increment);
                self.rect.set_x(self.rect.x + diagonal_increment);
            },

            Controller::Left => self.rect.set_x(self.rect.x - ortogonal_increment),
            Controller::Right => self.rect.set_x(self.rect.x + ortogonal_increment),
        };

        self.confine();
    }

    fn confine(&mut self){
        let speed = self.get_speed();

        let top = self.rect.top();
        let bottom = self.rect.bottom();
        let left = self.rect.left();
        let right = self.rect.right();
        let width = self.rect.width();
        let height = self.rect.height();



        let too_high = (top - speed) < 0;
        let too_low = (bottom + speed) > WINDOW_HEIGHT as i32;

        let too_left = (left - speed) < 0;
        let too_right = (right + speed) > WINDOW_WIDTH as i32;

        if too_high{
            self.rect.set_y(0)
        }
        if too_low {
            self.rect.set_y(WINDOW_HEIGHT as i32 - height as i32)
        }
        if too_left{
            self.rect.set_x(0)
        }
        if too_right{
            self.rect.set_x(WINDOW_WIDTH as i32 - width as i32)
        }
    }
    pub fn default_player(name: String) -> Self {
        let abs_speed: i32= 100; // pixel/sec
        // pixel/sec / frame/sec = pixel/frame * sec/sec = pixel/frame
        let speed = abs_speed / FPS as i32; // pixel/frame  (1 frame is around 1 loop cycle)
        let coor = ((WINDOW_WIDTH / 2) as i32, (WINDOW_HEIGHT / 2) as i32);
        Character {
            name,
            hp: 100,
            rect: Rect::new(coor.0, coor.1, 100, 100), // Default rect dimensions
            color: Color::BLUE,                        // Default color
            speed,
        }
    }

    pub fn increment_speed(&mut self, pixels_second: i32){
        if pixels_second >= FPS as i32{ 
            self.speed += pixels_second / FPS as i32;
        } else{
            self.speed = 1;
        }
        println!("{}", self.speed);
    }
    pub fn decrement_speed(&mut self, pixels_second: i32){
        if pixels_second >= FPS as i32{ 
            self.speed -= pixels_second / FPS as i32;
        } else{
            self.speed = 1;
        }
        println!("{}", self.speed);
    }
    pub fn get_rect(&self) -> Rect {
        self.rect
    }
    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
    fn get_speed(&self) -> i32{
        self.speed
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
}


