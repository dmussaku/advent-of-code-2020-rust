mod days;

use days::{day1};

fn main() {
    let day1_input = day1::day1::read_numbers_from_file("src/days/day1/input_files/file.txt");
    day1::day1::run(day1_input);
}
