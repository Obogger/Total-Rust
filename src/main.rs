use std::io::{self, Write};

mod list_typer;

mod utils;
use utils::*;


fn main() {
    command_helper::clear();
    const MAIN_MENU_MESSAGE: &str= "Wow hello, what are you doing here?\n";

    loop {
        command_helper::clear();

        println!("{:^50}", MAIN_MENU_MESSAGE);
        print!("> ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let trimmed_input = user_input.trim();
    
    
        match trimmed_input{
            "list_typer" => list_typer::main(),
            "-l" => list_projects(),
            "-q" => return,
            _ => help()
        }
    }
}

fn help(){
    command_helper::clear();
    println!("-l: List projects\n
-q: Quit");

    command_helper::pause();

}
fn list_projects() {
    command_helper::clear();

    println!("1. list_typer");


    command_helper::pause();
    return  
}