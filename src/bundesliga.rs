extern crate image;
use std::fs;

use window;

struct Files {
    imagePaths: Vec<String>,
}

impl Files {
    fn new(path: String) -> Files {
        let mut pathsVector: Vec<String> = Vec::new();

        for imagePath in fs::read_dir(&path).unwrap() {
            pathsVector.push(imagePath.unwrap().path().display().to_string());
        }
        Files{
            imagePaths: pathsVector,
        }
    }
}

struct Bundesliga {
    games: Vec<Game>,
    competitors: Vec<Competitor>,
}

#[derive(Debug)]
struct Game{
    player_home: Competitor,
    player_guest: Competitor,
    played: bool,
}

#[derive(Copy, Clone, Debug)]
struct Competitor {
    // path: std::fs::DirEntry,
    pathIndex: i32,
    points: i32,
}

impl Bundesliga {
    // setup the league
    fn setup() {

    }
    // run the league
    fn compete() {

    }
}

pub fn main(path: String) {
    println!("Directory: {:?}", path);

    let mut league = Bundesliga { games: vec![], competitors: vec![] };

    let files = Files::new(path);

    // make every image a competitor
    for (i, file) in files.imagePaths.iter().enumerate() {
        // println!("Img Path: {:?}", file);
        let mut competitor = Competitor{pathIndex: i as i32, points: 0};
        league.competitors.push(competitor)
    }

    // create the games 
    for (i, competitor) in league.competitors.iter().enumerate() {
        for j in (i + 1)..league.competitors.len() {
            let mut game = Game {player_home: *competitor, player_guest: league.competitors[j], played: false};
            // println!("Game: {:?}", game)
            league.games.push(game)
        }
    }

    for game in league.games {
        let image_left = image::open(&files.imagePaths[game.player_home.pathIndex as usize]);
        let image_right = image::open(&files.imagePaths[game.player_guest.pathIndex as usize]);

        match (image_left, image_right) {
            (Ok(image_left_result), Ok(image_right_result)) => {
                let error = window::draw_uwufufu(image_left_result, image_right_result);                        
            }
            _ => {
                println!("Something went wrong with your images :)");
            }
        }
    } 
}