use std::collections::BTreeMap;

use aoc2023::day_two::{get_game, get_game_part2, parse_input};

fn main() {
    // let input = include_str!("../../data/input2.txt");

    // let result1 = get_game(input);
    // let result2 = get_game_part2(input);

    // println!("day part1 result : {} {}", result1, result2);

    //nom parsing

    let input = include_str!("../../data/input2.txt");
    let games = parse_input(input).unwrap();

    //println!("parsed input {:?}", games);

    let map: BTreeMap<&str, u32> = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let count = games
        .1
        .iter()
        .filter_map(|game| game.is_valid_for_cubes(&map))
        .sum::<u32>();

    println!("{}", count);
}

#[cfg(test)]
mod tests_day_two {

    use crate::{get_game, get_game_part2};

    #[test]
    fn test_part2_should_return_2286() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let value = get_game_part2(line);
        assert_eq!(2286, value);
    }

    #[test]
    fn test_part1_should_return_8() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let value = get_game(line);
        assert_eq!(8, value);
    }
}
