use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::Screen;
use crate::window::Window;
use std::error::Error;

pub struct Display {
    conn: RustConnection,
    screen_num: usize,
    screen: Screen,
}

impl Display {
    pub fn open() -> Result<Self, Box<dyn Error>> {
        let (conn, screen_num) = RustConnection::connect(None)?;
        let screen = conn.setup().roots[screen_num].clone();

        Ok(Self { conn, screen_num, screen})
    }

    pub fn create_window(&self) -> Window {
        Window::default(&self.conn, self.screen.clone())
    }

    pub fn winHeight(&self) -> u16{
        return self.screen.height_in_pixels;
    }

    pub fn winWidth(&self) -> u16{
        return self.screen.width_in_pixels;
    }
}
