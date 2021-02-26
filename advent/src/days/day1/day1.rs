/*

Before you leave, the Elves in accounting just need you to fix your expense report
(your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and
then multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456
In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together
produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to 2020;
 what do you get if you multiply them together?
 */

use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn read_numbers_from_file(path: &str) -> Vec<i32>{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    let numbers: Vec<i32> = contents.lines()
        .map(|val| val.parse::<i32>().expect("parse error"))
        .collect();
    numbers
}

pub fn run(input_numbers: Vec<i32>) -> i32 {

    // We create a second vector by subtracting values from 2020
    let transformed_numbers: Vec<_> = input_numbers.iter()
        .map(|val| 2020 - *val)
        .collect();

    // Transform to HashSet
    let input_set: HashSet<i32> = HashSet::from_iter(input_numbers.iter().cloned());
    println!("Input Set");
    for num in &input_set {
        print!("{} ", num)
    }
    println!();

    println!("Transformed set");
    let transformed_set: HashSet<i32> = HashSet::from_iter(transformed_numbers.iter().cloned());
    for num in &transformed_set{
        print!("{} ", num)
    }
    println!();

    let _intersection: HashSet<_> = input_set.intersection(&transformed_set).collect();
    println!("Intersection");

    for num in &_intersection{
        print!("{} ", num);
    }
    println!();

    let result: i32 = _intersection.iter()
        .fold(1, |acc, val| acc * **val);

    println!("Result = {}", result);
    result
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input_numbers() {
        let input_numbers = read_numbers_from_file(
            "src/days/day1/input_files/test_file.txt"
        );

        assert_eq!(vec![1721, 979, 366, 299, 675, 1456], input_numbers);
    }

    #[test]
    fn test_run() {
        assert_eq!(514579, run(vec![1721, 979, 366, 299, 675, 1456]));
    }
}

