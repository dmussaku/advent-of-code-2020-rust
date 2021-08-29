use std::fs;

fn parse_entry(line_vec: Vec<&str>) -> (u8, u8, char, Vec<char>){
    // example input: ["1-3", "a:", "abcde"]
    // result output: (1, 3, 'a', ['a', 'b', 'c', 'd', 'e'])

    // First we get the range of password length
    let p_size_range: Vec<u8> = line_vec[0].split('-')
            .map(|s| s.parse().unwrap())
            .collect();
    let (min, max): (u8, u8) = (p_size_range[0], p_size_range[1]);

    // Next we take the required character
    let required_char: char = line_vec[1].chars().next().unwrap();

    // And last the password itself
    let password: Vec<char> = line_vec[2].chars().collect();

    (min, max, required_char, password)
}

fn is_valid_password_by_char_count(min: u8, max: u8, required_char: char, password_vector: Vec<char>) -> bool{
    let required_char_num: u8 = password_vector.iter()
        .filter(|&c| *c == required_char).count() as u8;

    required_char_num >= min && required_char_num <= max
}

fn is_valid_password_by_char_pos(first_pos: u8, last_pos: u8, required_char: char, password_vector: Vec<char>) -> bool{
    let is_at_first_pos: bool = password_vector[first_pos as usize - 1] == required_char;
    let is_at_last_pos: bool = password_vector[last_pos as usize - 1] == required_char;

    is_at_first_pos ^ is_at_last_pos
}

pub fn run_part1(path: &str) -> u16{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");

    let mut valid_password_count: u16 = 0;
    for line in contents.lines(){
        let input_line: Vec<&str> = line.split_whitespace().collect();
        // println!("{:?}", &input_line);
        let (min, max, required_char, password) = parse_entry(input_line);
        let is_valid_password: bool = is_valid_password_by_char_count(min, max, required_char, password);
        if is_valid_password == true{
            valid_password_count += 1;
        }
    }

    valid_password_count
}

pub fn run_part2(path: &str) -> u16 {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");

    let mut valid_password_count: u16 = 0;
    for line in contents.lines(){
        let input_line: Vec<&str> = line.split_whitespace().collect();
        let (first_pos, last_pos, required_char, password) = parse_entry(input_line);
        let is_valid_password: bool = is_valid_password_by_char_pos(first_pos, last_pos, required_char, password);
        if is_valid_password == true {
            valid_password_count += 1
        }
    }
    valid_password_count
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_parse_entry(){
        let input_line: Vec<&str> = vec!["1-3", "a:", "abcde"];

        assert_eq!((1, 3, 'a', vec!['a', 'b', 'c', 'd', 'e']), parse_entry(input_line));
    }

    #[rstest]
    #[case(1, 3, 'a', vec!['a', 'b', 'c', 'd', 'e'], true)]
    #[case(1, 3, 'b', vec!['c', 'd', 'e', 'f', 'g'], false)]
    fn test_is_valid_password_by_char_count(#[case] min: u8,
                              #[case] max: u8,
                              #[case] required_char: char,
                              #[case] password: Vec<char>,
                              #[case] result: bool) {
        assert_eq!(result, is_valid_password_by_char_count(min, max, required_char, password));
    }

    #[rstest]
    #[case(1, 3, 'a', vec!['a', 'b', 'c', 'd', 'e'], true)]
    #[case(1, 3, 'b', vec!['c', 'd', 'e', 'f', 'g'], false)]
    fn test_is_valid_password_by_char_pos(#[case] min: u8,
                              #[case] max: u8,
                              #[case] required_char: char,
                              #[case] password: Vec<char>,
                              #[case] result: bool) {
        assert_eq!(result, is_valid_password_by_char_pos(min, max, required_char, password));
    }

    #[test]
    fn test_run_part1() {
        let input_numbers = run_part1(
            "src/days/day2/input_files/test_file.txt"
        );

        assert_eq!(2, input_numbers);
    }

    #[test]
    fn test_run_part2() {
        let input_numbers = run_part2(
            "src/days/day2/input_files/test_file.txt"
        );

        assert_eq!(1, input_numbers);
    }
}
