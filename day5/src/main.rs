mod line;
mod parser;

use line::Line;
use line::Point;
use std::collections::HashMap;
use std::slice::Iter;

fn filtered_results(lines: Iter<Line>) {
    let mut results_map: HashMap<Point, usize> = HashMap::new();
    lines
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .flat_map(|line| line.coverage())
        .for_each(|p| {
            let count = results_map.entry(p).or_insert(0);
            *count += 1;
        });
    println!(
        "Filtered: {}",
        results_map.iter().filter(|(_, &v)| v > 1).count()
    );
}

fn full_results(lines: Iter<Line>) {
    let mut results_map: HashMap<Point, usize> = HashMap::new();
    lines.flat_map(|line| line.coverage()).for_each(|p| {
        let count = results_map.entry(p).or_insert(0);
        *count += 1;
    });
    println!(
        "Full result: {}",
        results_map.iter().filter(|(_, &v)| v > 1).count()
    );
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Could not read input");
    let lines = parser::lines_parser::file(&input).expect("Error parsing input");

    filtered_results(lines.iter());
    full_results(lines.iter());
}
