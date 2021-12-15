struct Tile {
    risk: u32,
    min_risk: u32,
    visited: bool,
}

impl From<u32> for Tile {
    fn from(risk: u32) -> Tile {
        Tile::new(risk)
    }
}

impl Tile {
    fn new(risk: u32) -> Self {
        Self {
            risk,
            min_risk: u32::MAX,
            visited: false,
        }
    }
}

struct Cavemap {
    map: Vec<Vec<Tile>>,
}

fn dequeue<T>(queue: &mut Vec<T>) -> Option<T> {
    if queue.is_empty() {
        None
    } else {
        Some(queue.remove(0))
    }
}

impl Cavemap {
    pub fn visit(&mut self) -> u32 {
        self.get_mut(0, 0).min_risk = 0;
        let mut tiles_to_visit = vec![(0, 0)];
        while let Some((x, y)) = dequeue(&mut tiles_to_visit) {
            self.get_mut(x, y).visited = true;
            self.neighbors(x, y).into_iter().for_each(|(nx, ny)| {
                let move_risk = self.get(nx, ny).risk + self.get(x, y).min_risk;
                let current_min_risk = self.get(nx, ny).min_risk;
                if current_min_risk > move_risk {
                    self.get_mut(nx, ny).min_risk = move_risk;
                    tiles_to_visit.push((nx, ny));
                }
            });
        }

        self.exit().min_risk
    }

    fn get(&self, x: i64, y: i64) -> &Tile {
        &self.map[y as usize][x as usize]
    }

    fn get_mut(&mut self, x: i64, y: i64) -> &mut Tile {
        &mut self.map[y as usize][x as usize]
    }

    fn neighbors(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x < (self.width() - 1) as i64 {
            neighbors.push((x + 1, y));
        }
        if y < (self.height() - 1) as i64 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    fn unvisited_neighbors(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        self.neighbors(x, y)
            .into_iter()
            .filter(|(nx, ny)| !self.get(*nx, *ny).visited)
            .collect()
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn exit(&self) -> &Tile {
        self.get((self.width() - 1) as i64, (self.height() - 1) as i64)
    }

    fn multiply(self, amount: u32) -> Self {
        let mut new_map = vec![];
        for y_incr in 0..amount {
            for row in &self.map {
                let mut new_row: Vec<Tile> = vec![];
                for x_incr in 0..amount {
                    // x % 9 + x_incr
                    for elem in row {
                        let risk = elem.risk;
                        let mut new_risk = risk + x_incr + y_incr;
                        if new_risk > 9 {
                            new_risk = new_risk - 9
                        }
                        new_row.push(new_risk.into())
                    }
                }
                new_map.push(new_row);
            }
        }

        Cavemap { map: new_map }
    }
}

fn parse_input(input: &str) -> Cavemap {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().into())
                .collect::<Vec<Tile>>()
        })
        .collect();

    Cavemap { map }
}

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> u32 {
    let input = std::fs::read_to_string("input").expect("Unable to read file");
    let mut cavemap = parse_input(&input);
    cavemap.visit()
}

fn part_2() -> u32 {
    let input = std::fs::read_to_string("input").expect("Unable to read file");
    let cavemap = parse_input(&input);
    let mut cavemap = cavemap.multiply(5);

    cavemap.visit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit() {
        let input = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";
        let mut map = parse_input(input);

        assert_eq!(map.visit(), 40);
    }

    #[test]
    fn test_part_1() {
        let result = part_1();

        assert_eq!(result, 487);
    }

    #[test]
    fn test_multiply() {
        let input = "12\n89";
        let cave = parse_input(input);
        let mut cave = cave.multiply(3);
        let output: Vec<Vec<u32>> = cave
            .map
            .iter()
            .map(|row| row.iter().map(|tile| tile.risk).collect())
            .collect();
        output.iter().for_each(|row| println!("{:?}", row));
    }
}
