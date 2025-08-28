mod color;
mod display;
mod window;
use crate::color::*;
use crate::display::*;
use crate::window::*;
use mlua::{Lua, Result};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::{fs, thread, time::Duration};

fn main() {
    let lua = Lua::new();
    let script_path = "script.lua";
    load_lua_script(&lua, script_path).unwrap();

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher =
        RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
    watcher
        .watch(Path::new(script_path), RecursiveMode::NonRecursive)
        .unwrap();

    let display = Display::open().unwrap();

    let window = display
        .create_window()
        .size(display.win_width(), 100)
        .title("My Rust X11 Window")
        .background(Color::Black)
        .show()
        .unwrap();

    //window.draw_line((50, 50), (200, 200), Color::Red)?;
    //window.draw_text((150, 150), "Hello, X11!", Color::Black)?;

    loop { 
        window
        .draw_rect((0, 0), (display.win_width(), 100), 0, 0, 255)
        .unwrap();

        while let Ok(res) = rx.try_recv() {
            if let Ok(event) = res {
                match event.kind {
                    notify::EventKind::Modify(_) | notify::EventKind::Create(_) => {
                        if let Err(e) = load_lua_script(&lua, script_path) {
                            eprintln!("Failed to reload Lua: {:?}", e);
                        }
                    }
                    _ => {}
                }
            } else {
                // Optional: handle notify::Error
                eprintln!("Watcher error: {:?}", res.err());
            }
        }

        let x1: i16 = lua.globals().get("x1").unwrap_or(100);
        let y1: i16 = lua.globals().get("y1").unwrap_or(100);
        let x2: i16 = lua.globals().get("x2").unwrap_or(100);
        let y2: i16 = lua.globals().get("y3").unwrap_or(100);
        let red: u8 = lua.globals().get("red").unwrap_or(100);
        let green: u8 = lua.globals().get("green").unwrap_or(100);
        let blue: u8 = lua.globals().get("blue").unwrap_or(100);
        let delay: u64 = lua.globals().get("delay").unwrap_or(500);

        if let Ok(update) = lua.globals().get::<_, mlua::Function>("update") {
            let _ = update.call::<_, ()>(());
        }

        window.draw_rect((x1, y1), (100, 25), red,green,blue).unwrap();
        window.draw_rect((x2, y2), (100, 25),red,green,blue).unwrap();

        thread::sleep(Duration::from_millis(delay));
    }
}

fn load_lua_script(lua: &Lua, path: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    lua.load(&content).exec()?;
    Ok(())
}
