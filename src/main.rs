use std::fs;

extern crate docopt;
extern crate rustc_serialize;
extern crate image;
extern crate minifb;

mod command_line;
mod window;
mod image_to_window;

fn main() {
    let args = command_line::read_command_line();
    // get path
    let path = args.arg_path;

    // THIS IS HORRIBLE
    let mut first_image_path = String::from("");
    let mut second_image_path = String::from("");

    let mut paths = fs::read_dir(&path).unwrap().take(1);
    for p in paths {
        first_image_path = p.unwrap().path().display().to_string();
    }

    paths = fs::read_dir(&path).unwrap().take(2);
    for p in paths {
        second_image_path = p.unwrap().path().display().to_string();
    }

    let image_left = image::open(&first_image_path);
    let image_right = image::open(&second_image_path);
    
    match &image_left {
        Ok(image) => {
            let error = window::draw_uwufufu(image_left.unwrap(), image_right.unwrap());
            if let Err(err) = error {
                println!("Error while opening window: {}", err);
            }
        },
        Err(error) => {
            println!("Error while loading image: {}", error);
            return;
        }
    };
}
 