use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let screen_width = screen.width_in_pixels;
    let bar_height = 30;

    // Create a thin bar window at the top of the screen
    let win = conn.generate_id()?;
    conn.create_window(
        0,
        win,
        screen.root,
        0, 0,                    // x, y
        screen_width, bar_height, // width, height
        0,                        // border width
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new()
            .override_redirect(1) // ignore WM
            .background_pixel(screen.white_pixel), // can be replaced with any color
    )?;

    conn.map_window(win)?;
    conn.flush()?;

    // Draw a black rectangle inside the bar
    let gc = conn.generate_id()?;
    conn.create_gc(gc, win, &CreateGCAux::new().foreground(screen.black_pixel))?;
    conn.poly_fill_rectangle(
        win,
        gc,
        &[Rectangle { x: 0, y: 0, width: screen_width, height: bar_height }],
    )?;

    conn.flush()?;

    // Keep running
    loop {
        conn.wait_for_event()?;
    }
}
