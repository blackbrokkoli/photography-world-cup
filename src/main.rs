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

    let image_option = image::open(&path);
    
    match image_option {
        Ok(image) => {
            let error = window::display_image(image);
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
 