
pub struct Record {
    red: usize,
    green: usize,
    blue: usize,
}

impl Record {
    fn new() -> Self{
        Self{
            red:0,
            green:0,
            blue:0,
        }
    }
}

pub struct Game {
    id: usize,
    subsets: Vec<Record>,
}

impl Game {
    fn new() -> Self {
        Self { id: 0, subsets: vec![] }
    }
}

pub mod day_two {
    use crate::{Record, Game};

    pub fn get_game(line : &str) -> usize {
        let game:Vec<&str> = line.split(":").collect();
        let game_id = game[0].split_ascii_whitespace();
        let game_id = game_id.last().unwrap().trim().parse::<usize>().expect("Error parsing game id");
        let subsets: Vec<&str> = game[1].split(";").collect();

        let mut the_game = Game::new();

        the_game.id = game_id;


        for record in subsets {
            let mut record_game = Record::new();
            for colour in record.split(",").into_iter(){
                let mut colour = colour.split_ascii_whitespace();
                if let Some(c) = colour.nth(1) {
                    match c {
                        "blue" => record_game.blue = colour.nth(0).unwrap().trim().parse::<usize>().expect("Error parsing colour number."),
                        "red" => record_game.red = colour.nth(0).unwrap().trim().parse::<usize>().expect("Error parsing colour number."),
                        "green" => record_game.green = colour.nth(0).unwrap().trim().parse::<usize>().expect("Error parsing colour number."),
                        _ => panic!(),
                    }
                }
            }
           //
            the_game.subsets.push(record_game);
        }

        0
        
    }

    fn explore_with_regex() {
        use regex::Regex;
        let pattern = r"?<game_id>([0-9]{1,3}}: ([0-9]{1,3} [\w]{3,5})+(\;)?"; 
        let my_regex = Regex::new(pattern);
    }
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
