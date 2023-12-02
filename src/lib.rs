use std::fs::read_to_string;

/// reads input file and returns content as Result<String, String>
pub fn read_input(path: &String) -> Result<String, String> {
    match read_to_string(path) {
        Ok(line) => Ok(line),
        Err(_) => Err(format!("Error reading file {}", &path)),
    }
}

pub mod module_day1 {

    /// Santa is trying to deliver presents in a large apartment building, but he can't find the right floor -
    /// the directions he got are a little confusing.
    /// He starts on the ground floor (floor 0) and then follows the instructions one character at a time.
    /// An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.
    /// The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.
    ///
    pub fn find_floor(content: &String) -> Option<i32> {
        let mut floor = 0;

        for c in content.chars() {
            if c.eq(&'(') {
                floor += 1;
            } else {
                floor -= 1;
            }
        }

        return Some(floor);
    }

    /// Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1).
    /// The first character in the instructions has position 1, the second character has position 2, and so on.
    pub fn find_basement_pos(content: &String) -> Option<i32> {
        let mut position: i32 = 0;
        let mut floor = 0;

        for c in content.chars() {
            if c.eq(&'(') {
                floor += 1;
            } else {
                floor -= 1;
            }

            position += 1;

            if floor == -1 {
                break;
            }
        }

        Some(position)
    }
}
