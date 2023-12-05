use adventcode::{
    module_day2::{ribbon_feet, square_feet, Dimension},
    read_input,
};
//Puzzle answer was 1606483
//Puzzle answer was 3842356
fn main() {
    let content = read_input(&"data/input2.txt".to_owned());
    let content = match content {
        Ok(content) => content,
        Err(_) => panic!("Error reading input!"),
    };

    let box_list = Box::new(content.lines().map(|line| {
        let xyz_vec: Vec<&str> = line.split('x').collect();
        Dimension {
            length: xyz_vec[0].to_string().parse::<i32>().unwrap(),
            width: xyz_vec[1].to_string().parse::<i32>().unwrap(),
            height: xyz_vec[2].to_string().parse::<i32>().unwrap(),
        }
    }));

    let square_list = box_list.clone().map(|dim| square_feet(dim).unwrap());

    let total_square_feet = square_list.reduce(|acc, e| acc + e);

    let ribbon_feet_list = box_list.map(|dim| ribbon_feet(dim).unwrap());

    let total_ribbon_feet = ribbon_feet_list.reduce(|acc, e| acc + e);

    println!("Total Square : {}", total_square_feet.unwrap());
    println!("Total ribbon feet : {}", total_ribbon_feet.unwrap());
}

#[cfg(test)]
mod tests_day2 {

    use crate::{ribbon_feet, square_feet, Dimension};

    /// A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon
    /// to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.
    #[test]
    fn test_the_feet_of_ribbon_needed_for_1x1x10_should_be_14() {
        let dim = Dimension {
            length: 1,
            width: 1,
            height: 10,
        };

        assert_eq!(Some(14), ribbon_feet(dim));
    }

    /// A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon
    /// to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet
    #[test]
    fn test_the_feet_of_ribbon_needed_for_2x3x4_should_be_34() {
        let dim = Dimension {
            length: 2,
            width: 3,
            height: 4,
        };

        assert_eq!(Some(34), ribbon_feet(dim));
    }

    /// A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of
    /// wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    #[test]
    fn test_square_feet_for_2x3x4_should_return_58() {
        let dim = Dimension {
            length: 2,
            width: 3,
            height: 4,
        };

        let square = square_feet(dim);

        assert_eq!(Some(58), square);
    }

    /// A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet
    /// of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
    #[test]
    fn test_square_feet_for_1x1x10_should_return_43() {
        let dim = Dimension {
            length: 1,
            width: 1,
            height: 10,
        };

        let square = square_feet(dim);

        assert_eq!(Some(43), square);
    }
}
