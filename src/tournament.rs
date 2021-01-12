use std::fs;
use log::{debug};
use std::process;

use super::draw;

#[derive(Debug)]
pub enum TournamentType {
    WorldCup,
    League,
    Dating,
}

// TODO: what about rounds?
pub struct Tournament {
    pub paths: Vec<String>,
    pub players: Vec<Player>,
    pub current_game_index: usize,
    // world_cup, league, dating
    pub category: TournamentType,
    pub games: Vec<Game>,
    pub done: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Player {
    // path: std::fs::DirEntry,
    pub path_index: usize,
    // only relevant for world cup and dating Players
    pub is_in: bool,
    // only relevant for league
    pub score: i32,
}

#[derive(Debug)]
pub struct Game {
    pub player_home: usize,
    pub player_guest: Option<usize>,
    pub played: bool,
}

impl Tournament {
    pub fn new(path: String, category: TournamentType) -> Tournament {
        // generate the Players
        let mut tournament = Tournament {
            paths: vec![],
            games: vec![],
            current_game_index: 0,
            players: vec![],
            category,
            done: false,
        };

        // add all file paths into league
        for image_path in fs::read_dir(&path).unwrap() {
            tournament.paths.push(image_path.unwrap().path().display().to_string());
        }

        // make every image a Player
        for (i, _file) in tournament.paths.iter().enumerate() {
            // println!("Img Path: {:?}", file);
            let player = Player {
                path_index: i,
                is_in: true,
                score: 0,
                // buffer: vec![],
            };
            tournament.players.push(player);
        }
        tournament
    }

    // returns the player_home of the current game
    pub fn get_player_home_index(&self) -> usize {
        self.players[self.games[self.current_game_index].player_home].path_index
    }

    // returns the player_guest for the current game
    pub fn get_player_guest_index(&self) -> usize {
        self.players[self.games[self.current_game_index].player_guest.unwrap()].path_index
    }

    pub fn generate_round(&mut self) {
        // WORLD CUP ROUND
        match self.category {
            TournamentType::WorldCup => {
                for i in (0..(self.players.len() - 1)).step_by(2) {
                    // check if there is another game after the one we are currently looking at
                    // if yes, create a game with this and the next in the iteration
                    // if not, give the game a free pass to the next round because what else are you gonna do
                    if i + 1 < self.players.len() {
                        let game = Game {
                            player_home: i,
                            player_guest: Some(i+1),
                            played: false,
                        };
                        self.games.push(game);
                    }
                    // if the condition is false, the Player in question does not compete
                    // this means its bool condition of "is in" does not get touched at all,
                    // making it automatically eligible for the next round
                }
            },
            // BUNDESLIGA STYLE ROUND
            TournamentType::League => {
                // just make a game out of every possible combination of Players
                for (i, _player) in self.players.iter().enumerate() {
                    for j in (i + 1)..self.players.len() {
                        let game = Game {
                            player_home: i,
                            player_guest: Some(j),
                            played: false,
                        };
                        self.games.push(game);
                    }
                }
            },
            TournamentType::Dating => {
                // make every Player its own match
                for (i, _player) in self.players.iter().enumerate() {
                    let game = Game {
                        player_home: i,
                        player_guest: None,
                        played: false,
                    };
                    self.games.push(game);
                }
            }
        }

    }

    pub fn get_current_buffer(&self) -> draw::Buffer {
        let game_index = self.current_game_index;
        let game = &self.games[game_index];

        // check whether we have one or two Players and draw accordingly
        if game.player_guest.is_none()  {
            let player_index = self.get_player_home_index();
            let path= &self.paths[player_index];
            let image = image::open(path).unwrap();
            let buffer = draw::buffer_from_image(image);
            buffer
        } else {
            let player_home_index = self.get_player_home_index();
            let player_guest_index = self.get_player_guest_index();
            let path_left = &self.paths[player_home_index];
            let path_right = &self.paths[player_guest_index];
            let image_left = image::open(path_left).unwrap();
            let image_right = image::open(path_right).unwrap();
            let buffer = draw::buffer_from_two_images(image_left, image_right);
            buffer
        }
    }

    pub fn settle_game(&mut self, input: String) {
        debug!("Settle game of {:?}:", self.category);
        self.games[self.current_game_index].played = true;
        // left swipe has the opposite meaning in dating mode
        match &self.category {
            TournamentType::Dating => {
                if input == "left" {
                    let player_index = self.get_player_home_index();
                    let player = &mut self.players[player_index];
                    player.is_in = false;
                }
            },
            TournamentType::WorldCup => {
                if input == "left" {
                    // guest player has lost
                    let player_index = self.get_player_guest_index();
                    let player = &mut self.players[player_index];
                    player.is_in = false;
                    debug!("brrrrrr: {:}", player.is_in);
                } else {
                    // home player has lost
                    let player_index = self.get_player_home_index();
                    let player = &mut self.players[player_index];
                    player.is_in = false;
                }
            },
            TournamentType::League => {
                if input == "left" {
                    // guest player has lost
                    let player_index = self.get_player_home_index();
                    let player = &mut self.players[player_index];
                    player.score += 1;
                    debug!("Player {}: {}", player.path_index, player.score);
                } else {
                    // home player has lost
                    let player_index = self.get_player_guest_index();
                    let player = &mut self.players[player_index];
                    player.score += 1;
                    debug!("Player {}: {}", player.path_index, player.score);
                }
            }
        }
    }

    pub fn set_next_game(&mut self) {
        debug!("Set next game");
        let old_game = self.current_game_index.clone();
        for (i, game) in self.games.iter().enumerate() {
            if !game.played {
                self.current_game_index = i as usize;
                break;
            }
        }

        if self.current_game_index == old_game {
            // when we are playing dating or league, there is just one round and we can always end
            // when we are here. If we are doing world cup, we may need another round.

            match self.category {
                TournamentType::WorldCup => {
                    // update the player list to only contain players that are not out
                    let old_players = self.players.clone();
                    self.players = vec![];
                    for player in old_players {
                        if player.is_in {
                            self.players.push(player);
                            debug!("Player {} in new round!", player.path_index);
                        }
                    }
                    // if we have three players, we just switch to league mode
                    // why? because we are giving one player a free ticket into the next round if
                    // the total number is odd. This would be very unfair for a 3-player-knockout.
                    // Instead, we want everyone against everyone - which is just league mode.
                    if self.players.len() <= 3 {
                        self.category = TournamentType::League;
                    }
                    self.generate_round();
                    self.current_game_index = 0;
                    debug!("New World Cup Round generated - {} players!", self.players.len());

                    if self.players.len() <= 1 {
                        self.done = true;
                    }
                }
                _ => {
                    self.done = true;
                }
            }
        }
    }

    pub fn render_scoreboard(&self) {
        for player in &self.players {
            println!("Player {:?} -- Points: {:?}, In: {}", player.path_index, player.score, player.is_in);
        }
    }

    pub fn handle_key_press(&mut self, input: String) {
        self.settle_game(input);
        debug!("Game settled");
        self.set_next_game();
        // check whether its over
        if self.done {
            self.render_scoreboard();
            process::exit(0x0100);
        }
    }
}

