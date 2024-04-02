pub mod characters;
pub mod sdl_thing;
pub mod keyboard_controls;

use characters::Character;
use sdl2::{self, pixels::Color};
use std::time::Duration;

use sdl_thing::State;

fn main() {
    let mut my_window = sdl_thing::get_window();
    my_window.setup();

    let player = Character::default_player("ValmerWolf".to_string());
    my_window.add_characters(player);

    let mut i = 0;

    while let State::Continue = my_window.state {
        i = (i + 1) % 255;
        my_window.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        my_window.canvas.clear();

        my_window.messages_mapper();
        my_window.draw_characters();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
