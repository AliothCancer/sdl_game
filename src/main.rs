pub mod characters;
pub mod keyboard_controls;
pub mod sdl_thing;

use std::time::Duration;

use characters::player;
use sdl_thing::window::{SdlWindow, State};


const FPS: u16 = 60;
fn main() {
    let mut my_window = SdlWindow::new();
    my_window.setup();

    let player = player::create("ValmerWolf");
    my_window.add_characters(player);

    while let State::Continue = my_window.state {
        my_window.clear();
        my_window.update();
        my_window.draw_characters();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS as u32));
    }
}
