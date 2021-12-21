use std::collections::HashSet;
use std::ops::RangeInclusive;

type Point = (i64, i64);
type Algorithm = HashSet<u64>;

struct State {
    data: HashSet<Point>,
    infinite_status: bool,
    range_x: RangeInclusive<i64>,
    range_y: RangeInclusive<i64>,
}

impl State {
    fn print(&self) {
        for y in self.range_y.clone() {
            for x in self.range_x.clone() {
                if self.data.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    fn value(&self, x: i64, y: i64) -> u64 {
        let mut value = 0;
        for i in 0..9 {
            let (x, y) = (x + i % 3 - 1, y + i / 3 - 1);
            if self.state_at(x, y) {
                value += 1 << 8 - i;
            }
        }

        value
    }

    fn next_value(&self, algorithm: &Algorithm, x: i64, y: i64) -> bool {
        let value = self.value(x, y);
        algorithm.contains(&value)
    }

    fn state_at(&self, x: i64, y: i64) -> bool {
        if self.range_x.contains(&x) && self.range_y.contains(&y) {
            self.data.contains(&(x, y))
        } else {
            self.infinite_status
        }
    }

    fn apply(&self, algorithm: &Algorithm) -> Self {
        let mut data = HashSet::new();
        let range_x = (self.range_x.start() - 1)..=(self.range_x.end() + 1);
        let range_y = (self.range_y.start() - 1)..=(self.range_y.end() + 1);

        for y in range_y.clone() {
            for x in range_x.clone() {
                if self.next_value(&algorithm, x, y) {
                    data.insert((x, y));
                }
            }
        }

        let infinite_status = if self.infinite_status == false && algorithm.contains(&0) {
            true
        } else if self.infinite_status == true && !algorithm.contains(&511) {
            false
        } else {
            self.infinite_status
        };

        State {
            data,
            infinite_status,
            range_x,
            range_y,
        }
    }
}

fn main() {
    part_1();
    part_2();
}

fn parse_input(input: &str) -> (Algorithm, State) {
    let mut lines = input.lines();
    let mut algorithm = HashSet::new();
    lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .for_each(|(x, _)| {
            algorithm.insert(x as u64);
        });

    lines.next();
    let mut points = HashSet::new();
    let mut y = 0;
    while let Some(line) = lines.next() {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(x, _)| {
                points.insert((x as i64, y));
            });
        y += 1;
    }

    (
        algorithm,
        State {
            data: points,
            range_x: 0..=100,
            range_y: 0..=100,
            infinite_status: false,
        },
    )
}

fn part_1() {
    let input = std::fs::read_to_string("input").expect("Unable to read file");
    let (algorithm, state) = parse_input(&input);
    let mut state = state;
    for _ in 1..=2 {
        state = state.apply(&algorithm);
    }

    println!("{}", state.data.len());
}

fn part_2() {
    let input = std::fs::read_to_string("input").expect("Unable to read file");
    let (algorithm, state) = parse_input(&input);
    let mut state = state;
    for _ in 1..=50 {
        state = state.apply(&algorithm);
    }

    println!("{}", state.data.len());
    state.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> (Algorithm, State) {
        parse_input("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\n#..#.\n#....\n##..#\n..#..\n..###")
    }

    fn test_true_input() -> (Algorithm, State) {
        parse_input(&std::fs::read_to_string("input").expect("Unable to read file"))
    }

    #[test]
    fn test_parse_input() {
        let (algo, state) = test_input();

        assert_eq!(algo.len(), 238);
        assert_eq!(state.data.len(), 10);

        state.print();
    }

    #[test]
    fn test_value() {
        let (_algo, state) = test_input();

        assert_eq!(state.value(2, 2), 34)
    }

    #[test]
    fn test_next_value() {
        let (algo, state) = test_input();

        assert_eq!(state.next_value(&algo, 2, 2), true)
    }

    #[test]
    fn test_apply() {
        let (algo, state) = test_true_input();
        let state = state.apply(&algo);
        state.print();
        let state = state.apply(&algo);
        state.print();

        assert_eq!(state.data.len(), 35)
    }

    #[test]
    fn test_part_1() {
        part_1();
    }

    #[test]
    fn test_part_2() {
        part_2();
    }
}
