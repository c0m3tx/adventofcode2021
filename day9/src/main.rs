use std::ops::Mul;

struct Cell {
    value: i64,
    visited: bool,
}

impl Cell {
    fn new(value: i64) -> Self {
        Cell {
            value,
            visited: false,
        }
    }
}

struct HeightMap {
    data: Vec<Vec<Cell>>,
}

impl HeightMap {
    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data[0].len()
    }

    fn at(&self, x: i64, y: i64) -> Option<&Cell> {
        if y < 0 || y >= self.rows() as i64 {
            None
        } else {
            if x < 0 || x >= self.cols() as i64 {
                None
            } else {
                Some(&self.data[y as usize][x as usize])
            }
        }
    }

    fn at_mut(&mut self, x: i64, y: i64) -> Option<&mut Cell> {
        if y < 0 || y >= self.rows() as i64 {
            None
        } else {
            if x < 0 || x >= self.cols() as i64 {
                None
            } else {
                Some(&mut self.data[y as usize][x as usize])
            }
        }
    }

    fn neighbors(&self, x: i64, y: i64) -> Vec<&Cell> {
        let mut out = vec![];
        self.at(x - 1, y).map(|v| out.push(v));
        self.at(x + 1, y).map(|v| out.push(v));
        self.at(x, y - 1).map(|v| out.push(v));
        self.at(x, y + 1).map(|v| out.push(v));

        out
    }

    fn visit(&mut self, x: i64, y: i64) -> i64 {
        let cell = self.at_mut(x, y);
        if cell.is_none() {
            return 0;
        }

        let cell = cell.unwrap();
        if cell.visited || cell.value == 9 {
            return 0;
        }

        cell.visited = true;

        let other_visits = self.visit(x - 1, y)
            + self.visit(x + 1, y)
            + self.visit(x, y - 1)
            + self.visit(x, y + 1);

        other_visits + 1
    }

    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut out = vec![];
        (0..self.rows()).for_each(|y| {
            (0..self.cols()).for_each(|x| {
                let c = self
                    .at(x as i64, y as i64)
                    .expect(format!("Not able to get {},{}?", x, y).as_str());
                let neighbors = self.neighbors(x as i64, y as i64);
                if neighbors.iter().all(|n| n.value > c.value) {
                    out.push((x, y));
                }
            })
        });
        out
    }
}

fn main() {
    let mut data = load_data();

    println!("{}", part_1(&data));
    println!("{}", part_2(&mut data));
}

fn part_1(data: &HeightMap) -> i64 {
    data.low_points().into_iter().fold(0, |acc, (x, y)| {
        acc + data.at(x as i64, y as i64).unwrap().value + 1
    })
}

fn part_2(data: &mut HeightMap) -> i64 {
    let low_points = data.low_points();
    let mut sizes = low_points
        .into_iter()
        .map(|(x, y)| data.visit(x as i64, y as i64))
        .collect::<Vec<i64>>();
    sizes.sort();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
}

fn load_data() -> HeightMap {
    let input = std::fs::read_to_string("input").expect("Failed to read input");
    parse_input(&input)
}

fn parse_input(input: &str) -> HeightMap {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .map(|v| Cell::new(v))
                .collect::<Vec<Cell>>()
        })
        .collect();
    HeightMap { data }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse_input("2199943210\n3987894921\n9856789892\n8767896789\n9899965678");
        let risk_points = part_1(&input);
        assert_eq!(risk_points, 15);
    }

    #[test]
    fn test_visit() {
        let mut input = parse_input("2199943210\n3987894921\n9856789892\n8767896789\n9899965678");
        let visit = input.visit(1, 0);
        assert_eq!(visit, 3);
    }

    #[test]
    fn test_part_2() {
        let mut input = parse_input("2199943210\n3987894921\n9856789892\n8767896789\n9899965678");
        let largest_basins = part_2(&mut input);
        assert_eq!(largest_basins, 1134);
    }
}
