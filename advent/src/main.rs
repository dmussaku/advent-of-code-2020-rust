mod days;

use days::{day1, day2};

fn main() {
    let day1_input = day1::day1::read_numbers_from_file("src/days/day1/input_files/file.txt");
    println!("Day1 part 1 result = {}", day1::day1::run_part_1(day1_input));
    let day1_input = day1::day1::read_numbers_from_file("src/days/day1/input_files/file.txt");
    println!("Day1 part 2 result = {}", day1::day1::run_part_2(day1_input));

    println!("Day2 part 1 result = {}", day2::day2::run_part1("src/days/day2/input_files/file.txt"));
    println!("Day2 part 2 result = {}", day2::day2::run_part2("src/days/day2/input_files/file.txt"));
}
