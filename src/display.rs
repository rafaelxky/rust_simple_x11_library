// Display connection handling
use x11rb::rust_connection::RustConnection;
use std::{f32::consts::E, fmt::Error};

pub struct Display{
    conn: RustConnection,
    screen_num: usize,
}
impl Display {
    pub fn open() -> Result<Self, Box<dyn std::error::Error>>{
        let (conn, screen_num) = RustConnection::connect(None)?;
        Ok(Self {conn, screen_num})
    }
}