use aoc2023::{
    day_one::{extract_digits, replace_letter_by_digit},
    read_input,
};

fn main() {
    let sum: i32 = read_input(&"data/input1.txt".to_string())
        .lines()
        .map(|line| extract_digits(line.to_string()).unwrap())
        .sum();
    println!("total trebuchet part1 : {}", sum);

    let sum: i32 = read_input(&"data/input1b.txt".to_string())
        .lines()
        .map(|line| replace_letter_by_digit(line.to_string()).unwrap())
        .sum();
    println!("total trebuchet part2 : {}", sum);
}

#[cfg(test)]
mod tests_day1 {

    use aoc2023::day_one::replace_letter_by_digit;

    use crate::extract_digits;

    ///part2

    #[test]
    fn test_sixeightoneight_should_return_68() {
        let line = "sixeightoneight".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(68), number)
    }

    #[test]
    fn test_oneoneight_should_return_18() {
        let line = "oneoneight".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(18), number)
    }

    #[test]
    fn test_oneonetwo3one_should_return_11() {
        let line = "oneonetwo3one".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(11), number)
    }

    #[test]
    fn test_sevenfourseven_should_return_77() {
        let line = "sevenfourseven".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(77), number)
    }

    #[test]
    fn test_7one7_should_return_77() {
        let line = "7one7".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(77), number)
    }

    #[test]
    fn test_twoneighthree_should_return_23() {
        let line = "twoneighthree".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(23), number)
    }

    #[test]
    fn test_threeight_should_return_38() {
        let line = "threeight".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(38), number)
    }

    #[test]
    fn test_twone_should_return_21() {
        let line = "twone".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(21), number)
    }

    #[test]
    fn test_eightwo_should_return_82() {
        let line = "eightwo".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(82), number)
    }

    #[test]
    fn test_two1nine_should_return_29() {
        let line = "two1nine".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(29), number)
    }

    #[test]
    fn test_eightwothree_should_return_83() {
        let line = "eightwothree".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(83), number)
    }

    #[test]
    fn test_abcone2threexyz_should_return_13() {
        let line = "abcone2threexyz".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(13), number)
    }

    #[test]
    fn test_xtwone3four_should_return_24() {
        let line = "xtwone3four".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(24), number)
    }

    #[test]
    fn test_4nineeightseven2_should_return_42() {
        let line = "4nineeightseven2".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(42), number)
    }

    #[test]
    fn test_zoneight234_should_return_14() {
        let line = "zoneight234".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(14), number)
    }

    #[test]
    fn test_7pqrstsixteen_should_return_76() {
        let line = "7pqrstsixteen".to_string();
        let number = replace_letter_by_digit(line);
        assert_eq!(Some(76), number)
    }

    ///part1
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
