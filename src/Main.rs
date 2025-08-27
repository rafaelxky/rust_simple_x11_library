mod display;
mod window;
mod color;
use crate::display::*;
use crate::window::*;
use crate::color::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let display = Display::open()?;
    let var_file_path = "vars.txt"; 

    let window = display.create_window()
        .size(display.winWidth(),100)
        .title("My Rust X11 Window")
        .background(Color::White)
        .show()?;  

        // this bariable should be set from an external file 
    //window.draw_line((50, 50), (200, 200), Color::Red)?;
    window.draw_rect((100, 37), (100, 25), Color::Blue)?;
    window.draw_rect((300, 37), (100, 25), Color::Blue)?;
    //window.draw_text((150, 150), "Hello, X11!", Color::Black)?;

    loop {
        
    }
    return Ok(())
}