mod sparse_matrix;

use sparse_matrix::SparseMatrix;

use std::collections::HashSet;

pub struct Fold {
    direction: char,
    position: i64,
}

fn parse_input() -> (SparseMatrix, Vec<Fold>) {
    peg::parser! {
        grammar parser() for str {
            rule number() -> i64 = n:$(['0'..='9']+) { n.parse().unwrap() }

            pub rule coords() -> (i64, i64) = x:number() "," y:number() { (x, y) }
            pub rule fold() -> Fold = "fold along " d:$(['x' | 'y']) "=" x:number() { Fold{direction: d.chars().next().unwrap(), position: x} }
        }
    };

    let input = std::fs::read_to_string("input").expect("Unable to read file");
    let coords: HashSet<(i64, i64)> = input
        .lines()
        .filter_map(|line| parser::coords(line).ok())
        .collect();

    let folds: Vec<Fold> = input
        .lines()
        .filter_map(|line| parser::fold(line).ok())
        .collect();

    (SparseMatrix::new(coords), folds)
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let (matrix, folds) = parse_input();
    let fold = &folds[0];
    if fold.direction == 'x' {
        println!("{}", matrix.fold_horizontal(fold.position).dots());
    } else {
        println!("{}", matrix.fold_vertical(fold.position).dots());
    }
}

fn part_2() {
    let (matrix, folds) = parse_input();
    let mut matrix = matrix;
    for fold in folds {
        if fold.direction == 'x' {
            matrix = matrix.fold_horizontal(fold.position);
        } else {
            matrix = matrix.fold_vertical(fold.position);
        }
    }

    println!("{}", matrix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        parse_input();
    }
}
