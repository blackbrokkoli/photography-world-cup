struct Bundesliga {
    matches: Vec<Match>,
    competitors: Vec<Competitor>,
}

struct Match {
    player_home: Competitor,
    player_guest: Competitor,
    winner: Competitor,
}

struct Competitor {
    path: String,
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