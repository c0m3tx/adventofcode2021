use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    pub value: u64,
    pub extracted: bool,
}

impl From<u64> for Cell {
    fn from(v: u64) -> Cell {
        Cell {
            value: v,
            extracted: false,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let output = format!("{}{}", self.value, if self.extracted { "*" } else { "" });
        write!(f, "{:<4}", &output)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    cells: Vec<Cell>,
    pub already_won: bool,
}

impl From<Vec<u64>> for Board {
    fn from(v: Vec<u64>) -> Board {
        Board {
            cells: v.iter().map(|&c| c.into()).collect(),
            already_won: false,
        }
    }
}

impl Board {
    fn rows(&self) -> Vec<Vec<&Cell>> {
        (0..5)
            .map(|row| (0..5).map(|col| self.at(row, col)).collect())
            .collect()
    }

    fn cols(&self) -> Vec<Vec<&Cell>> {
        (0..5)
            .map(|col| (0..5).map(|row| self.at(row, col)).collect())
            .collect()
    }

    pub fn at(&self, row: usize, col: usize) -> &Cell {
        &self.cells[row * 5 + col]
    }

    pub fn extract(&mut self, value: u64) -> bool {
        match self.cells.iter_mut().find(|c| c.value == value) {
            None => false,
            Some(cell) => {
                cell.extracted = true;
                true
            }
        }
    }

    pub fn winning(&self) -> bool {
        self.rows()
            .iter()
            .chain(self.cols().iter())
            .any(|rowcol| rowcol.iter().all(|c| c.extracted))
    }

    pub fn calculate_score(&self) -> u64 {
        self.cells
            .iter()
            .filter(|c| !c.extracted)
            .fold(0, |acc, c| acc + c.value)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let output = self
            .rows()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| format!("{}", c))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! extract {
        ($board:expr, $($value:expr),+) => {
                $(
                    $board.extract($value);
                )*
        }
    }

    fn test_board() -> Board {
        Board::from(vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ])
    }

    #[test]
    fn test_rows() {
        let b = test_board();

        let rows = b.rows();
        let expected: Board = vec![14, 21, 17, 24, 4].into();
        assert_eq!(rows[0], expected.cells.iter().collect::<Vec<&Cell>>())
    }

    #[test]
    fn test_cols() {
        let b = test_board();

        let cols = b.cols();
        let expected: Board = vec![14, 10, 18, 22, 2].into();
        assert_eq!(cols[0], expected.cells.iter().collect::<Vec<&Cell>>())
    }

    #[test]
    fn test_extract() {
        let mut b = test_board();
        assert!(b.extract(14));
        assert!(b.cells[0].extracted);
    }

    #[test]
    fn test_winning_row() {
        let mut b = test_board();
        extract!(&mut b, 14, 10, 18, 22, 2);
        assert!(b.cells[0].extracted);
        assert!(b.winning());
    }

    #[test]
    fn test_calculate_score() {
        let mut b = test_board();
        extract!(&mut b, 14, 10, 18, 22, 2);
        assert_eq!(b.calculate_score(), 259);
    }

    #[test]
    fn test_display_cell() {
        let c = Cell {
            value: 14,
            extracted: true,
        };
        let output = format!("{}", c);
        assert_eq!(output, "14* ")
    }

    #[test]
    fn test_display_board() {
        let mut b = test_board();
        extract!(&mut b, 21, 10, 18, 13);
        let output = format!("{}", b);
        assert_eq!(
            "14  21* 17  24  4   \n10* 16  15  9   19  \n18* 8   23  26  20  \n22  11  13* 6   5   \n2   0   12  3   7   ",
            &output
        )
    }
}
