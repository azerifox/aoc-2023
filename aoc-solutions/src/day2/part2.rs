use std::cmp;

use crate::day2::part1;

pub fn solve(input: &str) -> Result<String, String> {
    let games = part1::parse_games(&input);

    let min_cubes_per_game = games.iter().map(|game| find_min_bag_setup(game));
    let product_sum: u32 = min_cubes_per_game
        .map(|(red_cubes, green_cubes, blue_cubes)| {
            red_cubes as u32 * green_cubes as u32 * blue_cubes as u32
        })
        .sum();

    Ok(product_sum.to_string())
}

fn find_min_bag_setup(game: &part1::Game) -> (u8, u8, u8) {
    game.rounds.iter().flat_map(|round| &round.cube_draws).fold(
        (u8::MIN, u8::MIN, u8::MIN),
        |(min_red, min_green, min_blue), draw| match draw.color {
            part1::Color::Red => (cmp::max(min_red, draw.cube_count), min_green, min_blue),
            part1::Color::Green => (min_red, cmp::max(min_green, draw.cube_count), min_blue),
            part1::Color::Blue => (min_red, min_green, cmp::max(min_blue, draw.cube_count)),
        },
    )
}
