extern crate image;
use std::fs;

use window;

struct Files {
    imagePaths: Vec<String>,
    images: Vec<image::DynamicImage>,
}

impl Files {
    fn new(path: String) -> Files {
        let mut pathsVector: Vec<String> = Vec::new();
        let mut imagesVector: Vec<image::DynamicImage> = Vec::new();

        for (i, imagePath) in fs::read_dir(path).unwrap().enumerate() {
            pathsVector.push(imagePath.unwrap().path().display().to_string());
            imagesVector.push(image::open(&pathsVector[i]).unwrap());
        }
        Files{
            imagePaths: pathsVector,
            images: imagesVector,
        }
    }
}

struct Image {
    file: image::DynamicImage,
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
        let image_left = &files.images[game.player_home.pathIndex as usize];
        let image_right = &files.images[game.player_guest.pathIndex as usize];
        // let image_right = image::open(&files.imagePaths[game.player_guest.pathIndex as usize]);


        let error = window::draw_uwufufu(image_left, image_right);                        

    } 
}