use std::fs::read_to_string;

pub fn read_input(path: &String) -> String {
    read_to_string(path).expect(format!("Error reading file {}", path).as_str())
}

pub mod day_one {

    pub fn extract_digits(line: String) -> Option<i32> {
        let digits: Vec<String> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_string())
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
