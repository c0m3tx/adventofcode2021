mod fishes;
use fishes::*;

fn load_input() -> [u64; 9] {
    let input = std::fs::read_to_string("input").expect("Unable to read file");
    let input = input.lines().next().expect("Unable to read first line");
    let input = input
        .split(",")
        .map(|x| x.parse::<u8>().expect("Unable to parse"))
        .collect::<Vec<u8>>();
    input_to_array(&input)
}

fn main() {
    let mut school = load_input();
    for _ in 0..256 {
        school = optimized_step(school);
    }

    println!("{}", school.iter().sum::<u64>())
}
