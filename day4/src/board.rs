#[derive(Debug, PartialEq)]
pub struct Cell {
    value: u64,
    extracted: bool,
}

impl Cell {
    pub fn status(&self) -> (u64, bool) {
        (self.value, self.extracted)
    }
}

impl From<u64> for Cell {
    fn from(v: u64) -> Cell {
        Cell {
            value: v,
            extracted: false,
        }
    }
}

#[derive(Debug, PartialEq)]
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
            Some(cell) => {
                cell.extracted = true;
                true
            }
            None => false,
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
            .map(|c| c.value)
            .sum()
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
}
