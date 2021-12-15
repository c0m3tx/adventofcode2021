struct Tile {
    risk: u32,
    visited: bool,
}

impl From<u32> for Tile {
    fn from(risk: u32) -> Tile {
        Self {
            risk,
            visited: false,
        }
    }
}

impl Tile {
    fn new(risk: u32) -> Self {
        Self::from(risk)
    }
}

struct Cavemap {
    map: Vec<Vec<Tile>>,
    min: u32,
}

impl Cavemap {
    pub fn visit(&mut self) -> Option<u32> {
        self.visit_cell(0, 0, 0).map(|x| x - self.get(0, 0).risk)
    }

    fn get(&self, x: i64, y: i64) -> &Tile {
        &self.map[y as usize][x as usize]
    }

    fn get_mut(&mut self, x: i64, y: i64) -> &mut Tile {
        &mut self.map[y as usize][x as usize]
    }

    fn unvisited_neighbors(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        let mut neighbors = vec![];
        if x > 0 && !self.get(x - 1, y).visited {
            neighbors.push((x - 1, y));
        }
        if y > 0 && !self.get(x, y - 1).visited {
            neighbors.push((x, y - 1));
        }
        if x < (self.width() - 1) as i64 && !self.get(x + 1, y).visited {
            neighbors.push((x + 1, y));
        }
        if y < (self.height() - 1) as i64 && !self.get(x, y + 1).visited {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    fn visit_cell(&mut self, x: i64, y: i64, count: u32) -> Option<u32> {
        let new_risk = count + self.get(x, y).risk;
        if new_risk > self.min {
            return None;
        }

        if x == (self.width() - 1) as i64 && y == (self.width() - 1) as i64 {
            return Some(new_risk);
        }
        let tile = self.get_mut(x, y);
        tile.visited = true;

        let values: Vec<u32> = self
            .unvisited_neighbors(x, y)
            .iter()
            .filter_map(|&(x, y)| {
                let risk = self.get(x, y).risk;
                self.visit_cell(x, y, count + risk)
            })
            .collect();

        let tile = self.get_mut(x, y);
        tile.visited = false;

        match values.into_iter().min() {
            Some(min) => {
                self.min = min;
                Some(min)
            }
            None => None,
        }
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
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

    Cavemap { map, min: u32::MAX }
}

fn main() {
    part_1();
}

fn part_1() {
    let input = std::fs::read_to_string("input").expect("Unable to read file");
    let mut cavemap = parse_input(&input);
    println!("{}", cavemap.visit().expect("Unable to visit cavemap?"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit() {
        let input = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";
        let mut map = parse_input(input);

        assert_eq!(map.visit(), Some(40));
    }
}
