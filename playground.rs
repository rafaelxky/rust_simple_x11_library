use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // write as Display::open
    // abstract in display struct
    let (conn, screen_num) = RustConnection::connect(None)?;
    // abstract 
    let screen = &conn.setup().roots[screen_num];

    let screen_width = screen.width_in_pixels;
    let bar_height = 30;

    // put in object propertie display
    let win = conn.generate_id()?;

    // abstract this
    conn.create_window(
        0,
        win,
        screen.root,
        0, 0,                    
        screen_width, bar_height, 
        0,                        
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new()
            .override_redirect(1) 
            .background_pixel(screen.white_pixel), 
    )?;

    // abstract
    conn.map_window(win)?;
    // abstract
    conn.flush()?;

    // move to struct
    let gc = conn.generate_id()?;
    // bundle with window.draw_rect() to change colors.
    conn.create_gc(gc, win, &CreateGCAux::new().foreground(screen.black_pixel))?;
    // change to method of window struct. Remove need to explicitly pass win and gc.
    conn.poly_fill_rectangle(
        win,
        gc,
        &[Rectangle { x: 0, y: 0, width: screen_width, height: bar_height }],
    )?;

    // abstract
    conn.flush()?;

    
    // abstract later, I need basic operations for now
    loop {
        conn.wait_for_event()?;
    }
}

// We need state management so we dont need to constantly pass variables such as gc and win 
// So we need to put that in structs to offer that abstraction
