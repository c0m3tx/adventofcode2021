use std::collections::HashSet;
use std::fmt::Display;

pub struct SparseMatrix {
    data: HashSet<(i64, i64)>,
}

impl SparseMatrix {
    pub fn new(coords: HashSet<(i64, i64)>) -> Self {
        Self { data: coords }
    }

    fn width(&self) -> i64 {
        self.data.iter().map(|(i, _)| *i).max().unwrap() + 1
    }

    fn height(&self) -> i64 {
        self.data.iter().map(|(_, j)| *j).max().unwrap() + 1
    }

    fn load(data: Vec<&str>) -> Self {
        let data = data
            .iter()
            .map(|x| {
                let mut pos = x.split(",");
                (
                    pos.next().unwrap().parse().unwrap(),
                    pos.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        SparseMatrix { data }
    }

    pub fn fold_vertical(self, v: i64) -> Self {
        let mut data = HashSet::new();
        for (x, y) in self.data {
            if y < v {
                data.insert((x, y));
            } else if y > v {
                let transposed = (x, v - (y - v));
                data.insert(transposed);
            }
        }

        SparseMatrix { data }
    }

    pub fn fold_horizontal(self, h: i64) -> Self {
        let mut data = HashSet::new();
        for (x, y) in self.data {
            if x < h {
                data.insert((x, y));
            } else if x > h {
                let transposed = (h - (x - h), y);
                data.insert(transposed);
            }
        }

        SparseMatrix { data }
    }

    pub fn dots(&self) -> usize {
        self.data.len()
    }
}

impl Display for SparseMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width();
        let height = self.height();
        let mut output = String::from("");
        for i in 0..height {
            for j in 0..width {
                if self.data.contains(&(j, i)) {
                    output.push_str("#");
                } else {
                    output.push_str(".");
                }
            }
            output.push_str("\n");
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_vertical() {
        let matrix = SparseMatrix::load(vec![
            "6,10", "0,14", "9,10", "0,3", "10,4", "4,11", "6,0", "6,12", "4,1", "0,13", "10,12",
            "3,4", "3,0", "8,4", "1,10", "2,14", "8,10", "9,0",
        ]);

        let matrix = matrix.fold_vertical(7);
        let output = format!("{}", matrix);
        let expected = "#.##..#..#.\n#...#......\n......#...#\n#...#......\n.#.#..#.###\n";

        assert_eq!(output, expected);
    }

    #[test]
    fn test_fold_horizontal() {
        let matrix = SparseMatrix::load(vec![
            "0,0", "2,0", "3,0", "6,0", "9,0", "0,1", "4,1", "6,2", "10,2", "0,3", "4,3", "1,4",
            "3,4", "6,4", "8,4", "9,4", "10,4",
        ]);

        let matrix = matrix.fold_horizontal(5);
        let output = format!("{}", matrix);
        let expected = "#####\n#...#\n#...#\n#...#\n#####\n";

        assert_eq!(output, expected);
    }
}
