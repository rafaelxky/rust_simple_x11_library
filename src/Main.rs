mod display;
mod window;
mod color;
use crate::display::*;
use crate::window::*;
use crate::color::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let display = Display::open()?;

    let window = display.create_window()
        .size(800, 600)
        .title("My Rust X11 Window")
        .background(Color::White)
        .show()?;  // Automatically maps the window

    //window.draw_line((50, 50), (200, 200), Color::Red)?;
    window.draw_rect((100, 100), (100, 100), Color::Blue)?;
    window.draw_rect((300, 100), (100, 100), Color::Blue)?;
    //window.draw_text((150, 150), "Hello, X11!", Color::Black)?;

    loop {
        
    }
    return Ok(())
}