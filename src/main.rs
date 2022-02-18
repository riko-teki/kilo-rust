use std::io::{stdin, stdout};
use std::{process, env};
use editor::Editor;
use terminal_io::EnableRawMode;
use key::ReadKey;

mod terminal_io;
mod editor;
mod window;
mod sys;
mod key;
mod row;

//const VERSION: &str = "0.0.1";

fn main() {
    let t = stdout().enable_raw_mode().unwrap(); 
    let mut editor = Editor::new();

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 { 
        let filename = &args[1];
        editor.open_file(&filename).unwrap();
    }
    
    for c in stdin().keys() {
        if let Ok(key::EditorKey::Ctrl(113)) = c { 
            t.suspend_raw_mode().unwrap();
            process::exit(0); 
        }
        editor.move_cursor(&c.unwrap());
        editor.refresh_screen();
    }
}

