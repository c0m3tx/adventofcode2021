mod line;
mod parser;

use line::Line;
use line::Point;
use std::collections::HashMap;

fn results<'a, I>(lines: I) -> usize
where
    I: Iterator<Item = &'a Line>,
{
    let mut results_map: HashMap<Point, usize> = HashMap::new();
    lines.flat_map(Line::coverage).for_each(|p| {
        let count = results_map.entry(p).or_insert(0);
        *count += 1;
    });
    results_map.iter().filter(|(_, &v)| v > 1).count()
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Could not read input");
    let lines = parser::lines_parser::file(&input).expect("Error parsing input");

    let filtered_results = results(
        lines
            .iter()
            .filter(|line| line.is_horizontal() || line.is_vertical()),
    );
    let full_results = results(lines.iter());

    println!("Filtered results: {}", filtered_results);
    println!("Full results: {}", full_results);
}
