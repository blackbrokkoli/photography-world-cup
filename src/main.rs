use minifb::{Key, KeyRepeat};
use log::{debug};
use std::env;
use std::process;
use crate::tournament::Tournament;
use native_dialog::{FileDialog, MessageDialog, MessageType};

mod draw;
mod tournament;

struct Args {
    match_type: tournament::TournamentType,
    from_directory: String,
    to_directory: String,
}

impl Args {
    fn new (mut args: env::Args) -> Result<Args, &'static str> {
        args.next();

        let from_directory = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive 'from' folder for pictures"),
        };

        let to_directory = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive 'to' folder for pictures"),
        };

        let match_type = match args.next() {
            Some(arg) => match &arg[..] {
                "dating" => tournament::TournamentType::Dating,
                "world_cup" => tournament::TournamentType::WorldCup,
                "league" => tournament::TournamentType::League,
                _ => return Err("Unrecognized match type"),
            },
            None => return Err("Didn't receive match type"),
        };

        Ok(Args {
            from_directory,
            to_directory,
            match_type,
        })
    }
}

fn main() {
    env_logger::init();

    // dialog boxes be like brrrrrrrrrrrrr
    // let result = FileDialog::new()
    //     .set_location("~/Desktop")
    //     .add_filter("PNG Image", &["png"])
    //     .add_filter("JPEG Image", &["jpg", "jpeg"])
    //     .show_open_single_dir()
    //     .unwrap();
    // let message = format!("{:#?}", result);

    // Build the GUI in General
    let mut window = draw::get_window();
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Read relevant data from CMD
    let arguments: Vec<String> = env::args().collect();
    let directory_path = arguments[1].clone();

    let category = match &arguments[2][..] {
        "dating" => tournament::TournamentType::Dating,
        "world_cup" => tournament::TournamentType::WorldCup,
        "league" => tournament::TournamentType::League,
        _ => {
            eprintln!("Invalid TournamentType: {}", arguments[2]);
            process::exit(1);
        }
    };

    // debug!("{:};{:}", directory_path, category);

    let mut tournament = tournament::Tournament::new(directory_path, category);
    tournament.generate_round();

    let mut buffer = tournament.get_current_buffer();


    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(
            &buffer.bytes,
            draw::BUFFER_WIDTH as usize,
            draw::BUFFER_HEIGHT as usize
        ).unwrap();

        // the user pressed a like/dislike key, so we want to save the result and load the next game
        if window.is_key_pressed(Key::A, KeyRepeat::No) {
            debug!("A Pressed!");
            tournament.handle_key_press("left".to_string());
            debug!("Keypress Handled");
            buffer = tournament.get_current_buffer();
        }
        if window.is_key_pressed(Key::D, KeyRepeat::No) {
            debug!("D Pressed!");
            tournament.handle_key_press("right".to_string());
            debug!("Keypress Handled");
            buffer = tournament.get_current_buffer();
        }
    }
}