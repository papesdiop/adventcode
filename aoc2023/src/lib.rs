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

    pub fn replace_letter_by_digit(mut line: String) -> Option<i32>{
        let letters = vec!["one","two","three","four","five","six","seven","eight","nine"];
        let mut i = 0;
        for letter in letters {
            i+=1;
            line = line.replace(letter, i.to_string().as_str());
        }
        extract_digits(line)
    }
}
