use crate::app::{App,Term};
use crate::modes::Mode;

#[allow(unused_variables,unused_assignments)]
pub fn handle_command(app: &mut App, terminal: &mut Term) {
    // Example usage: "q!" will force quit
    let mut force_command = false;
    let command = app.command.clone();
    let mut command_chars = command.chars();
    if command.starts_with(":0x") {
        match i64::from_str_radix(&command[3..], 16) {
            Ok(x) => {
                app.files[app.tabs_index].cursor.goto(x as usize);
            }
            Err(_e) => {
                // TODO: Add error messages
            }
        }
    }
    if command.starts_with(":e ") {
        if let Err(_e) = app.open(&command[3..]) {
            // TODO: Log error message for "failed to open file"
        }
    }
    match command.trim().as_ref() {
        ":bnext" => {
            app.tab_next();
        }
        ":bprev" => {
            app.tab_previous();
        }
        ":bd" => {
            app.files.remove(app.tabs_index);
            if app.tabs_index == app.files.len() {
                app.tabs_index -= 1;
            }
            if app.files.is_empty() {
                app.mode = Mode::Quit;
                return;
            }
        }
        _ => {
            match command_chars.next() {
                Some(':') => {
                    for (index, c) in command_chars.enumerate() {
                        match c {
                            'q' => {
                                // NOTE: Unless forced, quit may be undone later
                                app.mode = Mode::Quit;
                            }
                            'w' => {
                                // TODO: Handle writing file
                            }
                            'a' => {
                                // TODO: Handle marking all
                            }
                            '!' => {
                                if index == 0 {
                                    app.mode = Mode::Bash;
                                    app.command = command[2..].to_string();
                                    return;
                                }
                                else {
                                    force_command = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Some('/') => {

                }
                _ => {}
            }
        }
    }
    
    // TODO: Add checking if file needs to be saved + check for force quit
}
