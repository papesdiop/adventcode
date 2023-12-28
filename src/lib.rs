use std::{collections::BTreeMap, ops::Not};

#[derive(Debug, Clone, Copy)]
pub struct Record {
    red: usize,
    green: usize,
    blue: usize,
}

impl Record {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug)]
pub struct Game<'a> {
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}

impl<'a> Game<'a> {
    pub fn is_valid_for_cubes(&self, map: &BTreeMap<&str, u32>) -> Option<u32> {
        self.rounds
            .iter()
            .any(|round| {
                round
                    .iter()
                    .any(|cube| cube.amount > *map.get(cube.color).expect("a valide cube"))
            })
            .not()
            .then_some(
                self.id
                    .parse::<u32>()
                    .expect("game id should be parseable u32"),
            )
    }
}

impl<'a> Game<'a> {
    fn new() -> Self {
        Self {
            id: "",
            rounds: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

pub mod day_two {
    use std::collections::BTreeMap;

    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, digit1, line_ending},
        multi::separated_list1,
        sequence::{preceded, separated_pair},
        IResult,
    };

    use crate::{Cube, Game, Record,};

    //testing nom crate
    // Game 1: 20 green, 3 red, 2 blue; 9 red, 16 blue, 18 green; 6 blue, 19 red, 10 green; 12 red, 19 green, 11 blue
    pub fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
        let (input, games) = separated_list1(line_ending, game_parser)(input)?;
        Ok((input, games))
    }

    fn game_parser(input: &str) -> IResult<&str, Game> {
        // 20 green, 3 red, 2 blue; 9 red, 16 blue, 18 green; 6 blue, 19 red, 10 green; 12 red, 19 green, 11 blue // 1
        let (input, id) = preceded(tag("Game "), digit1)(input)?;
        // "" // ["20 green, 3 red, 2 blue", "9 red, 16 blue, 18 green", "6 blue, 19 red, 10 green",...]
        let (input, _) = tag(": ")(input)?;
        let (input, rounds) = separated_list1(tag("; "), round_parser)(input)?;
        Ok((input, Game { id, rounds }))
    }

    //"20 green, 3 red, 2 blue"
    fn round_parser(input: &str) -> IResult<&str, Vec<Cube>> {
        let (input, cubes) = separated_list1(tag(", "), cube_parser)(input)?;
        //dbg!(input);
        Ok((input, cubes))
    }

    //"20 green"
    fn cube_parser(input: &str) -> IResult<&str, Cube> {
        let (input, (amount, color)) =
            separated_pair(nom::character::complete::u32, tag(" "), alpha1)(input)?;
        Ok((input, Cube { amount, color }))
    }

    pub fn get_game_part2(line: &str) -> usize {
        let mut valid_games = vec![];

        for line in line.lines() {
            let mut max_game = Record::new();
            let game: Vec<&str> = line.split(":").collect();
            let game_id = game[0].split(" ");
            let game_id = game_id
                .last()
                .unwrap()
                .trim()
                .parse::<usize>()
                .expect("Error parsing game id");
            let subsets: Vec<&str> = game[1].split(";").collect();

            let mut the_game = Game::new();

            the_game.id = game_id.to_string().as_str();

            for record in subsets {
                for colour in record.trim().split(",").into_iter() {
                    let record_game = get_record(colour);
                    if max_game.blue < record_game.blue {
                        max_game.blue = record_game.blue;
                    }
                    if &max_game.red < &record_game.red {
                        max_game.red = record_game.red;
                    }
                    if &max_game.green < &record_game.green {
                        max_game.green = record_game.green;
                    }
                } //colours of each subset cubes
            } //game

            valid_games.push(max_game);
        }

        //dbg!(&valid_games);

        let result: usize = valid_games
            .iter()
            .map(|game| game.blue * game.red * game.green)
            .sum();

        result
    }

    pub fn get_game(input: &str) -> u32 {
        let games = parse_input(input).unwrap();

        //println!("parsed input {:?}", games);

        let map: BTreeMap<&str, u32> = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        games
            .1
            .iter()
            .filter_map(|game| game.is_valid_for_cubes(&map))
            .sum::<u32>()
    }

    fn get_record<'a>(colour: &'a str) -> Record {
        let mut record_game = Record::new();
        let mut colour = colour.trim().split(" ");
        let colour_cp = colour.clone();
        if let Some(c) = colour_cp.last() {
            match c {
                "blue" => {
                    record_game.blue = colour
                        .next()
                        .unwrap()
                        .trim()
                        .parse::<usize>()
                        .expect("Error parsing colour number.")
                }
                "red" => {
                    record_game.red = colour
                        .next()
                        .unwrap()
                        .trim()
                        .parse::<usize>()
                        .expect("Error parsing colour number.")
                }
                "green" => {
                    record_game.green = colour
                        .next()
                        .unwrap()
                        .trim()
                        .parse::<usize>()
                        .expect("Error parsing colour number.")
                }
                _ => panic!(),
            }
        }
        record_game
    }

    // fn explore_with_regex() {
    //     use regex::Regex;
    //     let pattern = r"?<game_id>([0-9]{1,3}}: ([0-9]{1,3} [\w]{3,5})+(\;)?";
    //     let my_regex = Regex::new(pattern);
    // }
}

pub mod day_one {

    pub fn extract_digits(line: String) -> Option<i32> {
        let digits: Vec<char> = line
            .chars()
            .filter(|c| c.is_digit(10))
            //.map(|c| c.to_string())
            .collect();
        let first = digits.first();
        let last = digits.last();
        if let (Some(first), Some(last)) = (first, last) {
            return Some(format!("{}{}", first, last).parse::<i32>().unwrap());
        }
        None
    }

    pub fn replace_letter_by_digit(mut line: String) -> Option<i32> {
        let letters = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut letters_vec = Vec::new();

        let line_cp = line.clone();
        for letter in letters {
            for each in line_cp.match_indices(letter) {
                letters_vec.push(each)
            }
        }

        letters_vec.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

        let letters_sorted = letters_vec;

        if !letters_sorted.is_empty() {
            let fletter = letters_sorted.first().unwrap().1;
            let mut freplacer = convert_to_digit(fletter).unwrap().to_string();
            freplacer.push_str(fletter);
            let lletter = letters_sorted.last().unwrap().1;
            line = line.replace(fletter, freplacer.as_str());
            line = line.replace(
                lletter,
                convert_to_digit(lletter).unwrap().to_string().as_str(),
            );
        }

        extract_digits(line)
    }

    fn convert_to_digit(letter: &str) -> Option<usize> {
        let number = match letter {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        };
        if number != 0 {
            return Some(number);
        }
        None
    }
}
