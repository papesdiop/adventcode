use adventcode::{read_input, module_day1::*};

/// puzzle answer was 232.
/// puzzle answer was 1783.
fn main() {
    let content = read_input(&"data/input1.txt".to_owned());
    let content = match content {
        Ok(content) => content,
        Err(_) => panic!("Error reading input!"),
    };
    let result = (find_floor(&content), find_basement_pos(&content));
    if let (Some(floor), Some(position)) = result {
        print!(
            "The floor is : {} and the basement position is : {}",
            floor, position
        );
    }
}

#[cfg(test)]
mod tests_day1 {


    use crate::{find_basement_pos, find_floor, read_input};

    #[test]
    fn should_able_to_read_file() {
        let content = read_input(&"data/input1.txt".to_string()).unwrap();
        assert!(content.len() > 0);
    }

    #[test]
    #[should_panic]
    fn should_able_fail_reading_unexisting_file() {
        read_input(&"this_doesnt_exist.txt".to_string()).unwrap();
    }

    #[test]
    fn test_floor_should_be_equal_to_zero() {
        let content = "(())";
        assert_eq!(Some(0), find_floor(&content.to_string()));
        let content = "()()";
        assert_eq!(Some(0), find_floor(&content.to_string()));
    }

    #[test]
    fn test_floor_should_be_equal_to_3() {
        let content = "(((";
        assert_eq!(Some(3), find_floor(&content.to_string()));
        let content = "(()(()(";
        assert_eq!(Some(3), find_floor(&content.to_string()));
        let content = "))(((((";
        assert_eq!(Some(3), find_floor(&content.to_string()));
    }

    #[test]
    fn test_floor_should_be_equal_to_minus_1() {
        let content = "())";
        assert_eq!(Some(-1), find_floor(&content.to_string()));
        let content = "))(";
        assert_eq!(Some(-1), find_floor(&content.to_string()));
    }

    #[test]
    fn test_floor_should_be_equal_to_minus_3() {
        let content = ")))";
        assert_eq!(Some(-3), find_floor(&content.to_string()));
        let content = ")())())";
        assert_eq!(Some(-3), find_floor(&content.to_string()));
    }

    #[test]
    fn test_basement_pos_should_be_equal_to_1() {
        let content = ")";
        assert_eq!(Some(1), find_basement_pos(&content.to_string()));
    }

    #[test]
    fn test_basement_pos_should_be_equal_to_5() {
        let content = "()())";
        assert_eq!(Some(5), find_basement_pos(&content.to_string()));
    }
}
