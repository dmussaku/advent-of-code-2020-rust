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

pub fn run_part_1(input_numbers: Vec<i32>) -> i32 {
    // We create a second vector by subtracting values from 2020
    let transformed_numbers: Vec<_> = input_numbers.iter()
        .map(|val| 2020 - *val)
        .collect();

    // Transform to HashSet
    let input_set: HashSet<i32> = HashSet::from_iter(input_numbers.iter().cloned());
    let transformed_set: HashSet<i32> = HashSet::from_iter(transformed_numbers.iter().cloned());

    let _intersection: HashSet<_> = input_set.intersection(&transformed_set).collect();

    let result: i32 = _intersection.iter()
        .fold(1, |acc, val| acc * **val);

    result
}

pub fn run_part_2(input_numbers: Vec<i32>) -> i32{
    // We create a second vector by subtracting values from 2020
    let transformed_numbers: Vec<i32> = input_numbers.iter()
        .map(|val| 2020 - *val)
        .collect();
    let transformed_set: HashSet<i32> = HashSet::from_iter(transformed_numbers);

    let mut result_set = HashSet::new();
    for i in 0..input_numbers.len(){
        for j in i+1..input_numbers.len(){
            let sum = input_numbers[i] + input_numbers[j];
            if sum < 2020{
                if transformed_set.contains(&sum){
                    result_set.insert(input_numbers[i]);
                    result_set.insert(input_numbers[j]);
                }
            }
        }
    }

    let result = result_set.iter().fold(1, |acc, val| acc * val);
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
    fn test_run_part_1() {
        assert_eq!(514579, run_part_1(vec![1721, 979, 366, 299, 675, 1456]));
    }

    #[test]
    fn test_run_part2() {
        assert_eq!(241861950, run_part_2(vec![1721, 979, 366, 299, 675, 1456]));
    }
}

