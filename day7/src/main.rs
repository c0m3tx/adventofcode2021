mod crabs;
use crabs::{find_best_incremental_fuel_consumption, find_best_linear_fuel_consumption, parse};

fn load_file() -> Vec<i64> {
    let input = std::fs::read_to_string("input").expect("File not found");
    parse(&input)
}

fn main() {
    let input = load_file();
    println!(
        "Best linear fuel consumption: {}",
        find_best_linear_fuel_consumption(&input)
    );

    println!(
        "Best incremental fuel consumption: {}",
        find_best_incremental_fuel_consumption(&input)
    );
}
