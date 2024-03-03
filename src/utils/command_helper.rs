use std::io::{stdin, stdout, Write};
use std::process::Command;

use termion::input::TermRead;
use termion::raw::IntoRawMode;



pub fn pause() {
    println!("\nPress something to go sback");
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    stdin().events().next();  
}

pub fn clear() {
    Command::new("clear").status().unwrap();
}