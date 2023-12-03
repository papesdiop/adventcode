use aoc2023::{day_one::extract_digits, read_input};

fn main() {
    let sum: i32 = read_input(&"data/input1.txt".to_string())
        .lines()
        .map(|line| extract_digits(line.to_string()).unwrap())
        .sum();
    println!("total trebuchet : {}", sum);
}

#[cfg(test)]
mod tests_day1 {

    use crate::extract_digits;

    #[test]
    fn test_1abc2_should_return_12() {
        let line = "1abc2".to_string();
        let number = extract_digits(line);
        assert_eq!(Some(12), number)
    }

    #[test]
    fn test_pqr3stu8vwx_should_return_38() {
        let line = "pqr3stu8vwx".to_string();
        let number = extract_digits(line);
        assert_eq!(Some(38), number)
    }

    #[test]
    fn test_a1b2c3d4e5f_should_return_15() {
        let line = "a1b2c3d4e5f".to_string();
        let number = extract_digits(line);
        assert_eq!(Some(15), number)
    }

    #[test]
    fn test_treb7uchet_should_return_77() {
        let line = "treb7uchet".to_string();
        let number = extract_digits(line);
        assert_eq!(Some(77), number)
    }
}
