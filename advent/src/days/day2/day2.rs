use std::fs;
// use std::collections::HashSet;

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

fn is_valid_password(min: u8, max: u8, required_char: char, password_vector: Vec<char>) -> bool{
    let required_char_num: u8 = password_vector.iter()
        .filter(|&c| *c == required_char).count() as u8;

    required_char_num >= min && required_char_num <= max
}

pub fn run(path: &str) -> u16{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");

    let mut valid_password_count: u16 = 0;
    for line in contents.lines(){
        let input_line: Vec<&str> = line.split_whitespace().collect();
        // println!("{:?}", &input_line);
        let (min, max, required_char, password) = parse_entry(input_line);
        let is_valid_password: bool = is_valid_password(min, max, required_char, password);
        if is_valid_password == true{
            valid_password_count += 1;
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
    fn test_is_valid_password(#[case] min: u8,
                              #[case] max: u8,
                              #[case] required_char: char,
                              #[case] password: Vec<char>,
                              #[case] result: bool) {
        assert_eq!(result, is_valid_password(min, max, required_char, password));
    }

    #[test]
    fn test_run() {
        let input_numbers = run(
            "src/days/day2/input_files/test_file.txt"
        );

        assert_eq!(2, input_numbers);
    }
}
