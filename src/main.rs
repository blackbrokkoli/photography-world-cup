use std::fs;

extern crate docopt;
extern crate rustc_serialize;
extern crate image;
extern crate minifb;

mod command_line;
mod window;
mod image_to_window;
mod bundesliga;

fn main() {
    let args = command_line::read_command_line();

    let directory_path = args.arg_path;
    
    bundesliga::main(directory_path);
}
 