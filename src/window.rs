use crate::color::*;
use std::str::FromStr;
use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ChangeGCAux, ConnectionExt, CreateGCAux, CreateWindowAux, Rectangle, Screen, WindowClass,
    },
    rust_connection::RustConnection,
};

pub struct Window<'a> {
    width: Option<u16>,
    height: Option<u16>,
    title: Option<String>,
    background: Option<Color>,
    conn: &'a RustConnection,
    win_id: u32,
    screen: Screen,
    gc: Option<u32>,
}

impl<'a> Window<'a> {
    pub fn default(conn: &'a RustConnection, screen: Screen) -> Self {
        Self {
            width: None,
            height: None,
            title: None,
            background: None,
            conn,
            win_id: conn.generate_id().expect("Failed to generate window id"),
            screen,
            gc: None,
        }
    }

    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(String::from(title));
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    pub fn show(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let width = self.width.unwrap_or(800);
        let height = self.height.unwrap_or(600);

        // Create the window
        self.conn.create_window(
            0,
            self.win_id,
            self.screen.root,
            0,
            0,
            width,
            height,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            // here is were we configure the win man controll (1 doesnt controll, remove override_redirect for controll)
            &CreateWindowAux::new()
                .override_redirect(1)
                .background_pixel(self.screen.black_pixel),
        )?;

        // Map (make visible)
        self.conn.map_window(self.win_id)?;
        self.conn.flush()?; // important!

        // Create GC for drawing
        let gc = self.conn.generate_id()?;
        self.conn.create_gc(
            gc,
            self.win_id,
            &CreateGCAux::new().foreground(self.screen.black_pixel),
        )?;

        self.gc = Some(gc);

        let font_id = self.conn.generate_id()?;
        self.conn.open_font(font_id, b"fixed")?;
        self.conn.change_gc(gc, &ChangeGCAux::new().font(font_id))?;

        Ok(self)
    }

    pub fn draw_rect(
        &self,
        (x, y): (i16, i16),
        (width, height): (u16, u16),
        red: u8,
        green: u8,
        blue: u8,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let color = self.alloc_rgb_pixel(red, green, blue)?;

        let gc = self.gc.expect("Window not shown (GC not created)");

        // FIXME: map your `Color` enum to an X11 pixel here
        self.conn
            .change_gc(gc, &ChangeGCAux::new().foreground(color))?;

        self.conn.poly_fill_rectangle(
            self.win_id,
            gc,
            &[Rectangle {
                x,
                y,
                width,
                height,
            }],
        )?;

        self.conn.flush()?;
        Ok(())
    }

    fn alloc_rgb_pixel(&self, r: u8, g: u8, b: u8) -> Result<u32, Box<dyn std::error::Error>> {
        let r = (r as u16) << 8;
        let g = (g as u16) << 8;
        let b = (b as u16) << 8;
        Ok(self
            .conn
            .alloc_color(self.screen.default_colormap, r, g, b)?
            .reply()?
            .pixel)
    }

    pub fn draw_text(
        &self,
        (x, y): (i16, i16),
        text: &str,
        red: u8,
        green: u8,
        blue: u8,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let color = self.alloc_rgb_pixel(red, green, blue)?;
        let gc = self.gc.expect("Window not shown (GC not created)");

        self.conn
            .change_gc(gc, &ChangeGCAux::new().foreground(color))?;
        self.conn
            .image_text8(self.win_id, gc, x, y, text.as_bytes())?;
        self.conn.flush()?;
        Ok(())
    }

    pub fn change_font(&self, font: &str) -> Result<(), Box<dyn std::error::Error>> {
        let font_id = self.conn.generate_id()?;
        self.conn.open_font(font_id, font.as_bytes())?;
        self.conn.change_gc(self.gc.unwrap(), &ChangeGCAux::new().font(font_id))?;

        Ok(())
    }
}
