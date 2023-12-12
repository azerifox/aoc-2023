pub fn solve(input: &str) -> Result<String, String> {
    let games = parse_games(input);

    let possible_games: Vec<&Game> = games
        .iter()
        .filter(|&game| {
            game.rounds.iter().all(|round| {
                round.cube_draws.iter().all(|draw| match draw.color {
                    Color::Red => draw.cube_count <= RED_CUBES,
                    Color::Green => draw.cube_count <= GREEN_CUBES,
                    Color::Blue => draw.cube_count <= BLUE_CUBES,
                })
            })
        })
        .collect();

    let game_id_sum: u32 = possible_games.iter().map(|&game| game.id as u32).sum();
    Ok(game_id_sum.to_string())
}

const RED_CUBES: u8 = 12;
const GREEN_CUBES: u8 = 13;
const BLUE_CUBES: u8 = 14;

#[derive(Debug)]
pub(super) struct Game {
    id: u8,
    pub(super) rounds: Vec<Round>,
}

impl Game {
    fn new(id: u8, rounds_input: &str) -> Game {
        let game_rounds = rounds_input
            .split(";")
            .map(|round_draws| Round::new(round_draws))
            .collect();
        Game {
            id,
            rounds: game_rounds,
        }
    }
}

#[derive(Debug)]
pub(super) struct Round {
    pub(super) cube_draws: Vec<Draw>,
}

impl Round {
    fn new(round_input: &str) -> Round {
        let draws: Vec<Draw> = round_input
            .split(",")
            .map(|color_draw| color_draw.trim())
            .map(|color_draw| {
                let draw: Vec<&str> = color_draw.split(" ").collect();
                match draw[1] {
                    "green" => Draw { color: Color::Green, cube_count: draw[0].parse().unwrap() },
                    "blue" => Draw { color: Color::Blue, cube_count: draw[0].parse().unwrap() },
                    "red" => Draw { color: Color::Red, cube_count: draw[0].parse().unwrap() },
                    _ => panic!("Unknown color but puzzle input should be fine and this should never happen")
                }
            })
            .collect();

        Round { cube_draws: draws }
    }
}

#[derive(Debug)]
pub(super) enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub(super) struct Draw {
    pub(super) color: Color,
    pub(super) cube_count: u8,
}

pub(super) fn parse_games(input: &str) -> Vec<Game> {
    input
        .split("\n")
        .filter(|&row| !row.is_empty())
        .map(|game| {
            let (game_id_part, rounds_part) = game.split_once(":").unwrap();
            let (_, game_id) = game_id_part.split_once(" ").unwrap();
            let game_id: u8 = game_id.parse().unwrap();
            Game::new(game_id, rounds_part)
        })
        .collect()
}
