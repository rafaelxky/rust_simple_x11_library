mod display;
use crate::display::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let display = Display::open()?;
    return Ok(())
}