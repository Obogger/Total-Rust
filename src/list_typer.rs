use std::io::{self, Write};

use crate::{command_helper, math};

const LINES_TO_DISPLAY: usize = 10;

pub fn main() {
    command_helper::clear();

    let mut to_print: Vec<String> = Vec::new();
    const MENU_MESSAGE: &str= "list_typer: Primary Test\n";

    loop{
        //let mut stdout = io::stdout();
        println!("{:^50}", MENU_MESSAGE);
        let list_index_start: i64 = to_print.len() as i64 - LINES_TO_DISPLAY as i64; 
        let mut list_index: usize = math::clamp(list_index_start, Some(0), None) as usize;
        for _ in 0..LINES_TO_DISPLAY{
            if list_index < to_print.len(){
                println!("{}", to_print[list_index]);
                list_index += 1;
            }
            else{
                println!("");
            }
        }
        for _ in 0..50{
            print!("-")
        }
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let trimmed_input = user_input.trim();
        if trimmed_input == "-q" {return}
        to_print.push(trimmed_input.to_string());
        command_helper::clear();
    }
}
