pub struct Line {
    pub start: Point,
    pub end: Point,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn offset(&self, dx: i64, dy: i64) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    #[allow(dead_code)]
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    #[allow(dead_code)]
    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn dx(&self) -> i64 {
        (self.end.x - self.start.x).signum()
    }

    fn dy(&self) -> i64 {
        (self.end.y - self.start.y).signum()
    }

    pub fn coverage(&self) -> Vec<Point> {
        let dx = self.dx();
        let dy = self.dy();

        let mut tracer = Point::new(self.start.x, self.start.y);
        let mut output = vec![tracer];
        while tracer != self.end {
            tracer = tracer.offset(dx, dy);
            output.push(tracer);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn test_coverage() {
        let line = parser::lines_parser::line("0,9 -> 5,9").unwrap();
        let cov = line.coverage();
        assert_eq!(cov.len(), 6)
    }

    #[test]
    fn test_coverage_reversed() {
        let line = parser::lines_parser::line("5,9 -> 0,9").unwrap();
        let cov = line.coverage();
        assert_eq!(cov.len(), 6)
    }

    #[test]
    fn test_coverage_vertical() {
        let line = parser::lines_parser::line("2,4 -> 2,9").unwrap();
        let cov = line.coverage();
        assert_eq!(cov.len(), 6)
    }

    #[test]
    fn test_coverage_vertical_reversed() {
        let line = parser::lines_parser::line("2,9 -> 2,4").unwrap();
        let cov = line.coverage();
        assert_eq!(cov.len(), 6)
    }

    #[test]
    fn test_coverage_diagonal() {
        let line = parser::lines_parser::line("1,1 -> 5,5").unwrap();
        let cov = line.coverage();
        assert_eq!(cov.len(), 5)
    }

    #[test]
    fn test_coverage_diagonal_2() {
        let line = parser::lines_parser::line("5,5 -> 1,1").unwrap();
        let cov = line.coverage();
        assert_eq!(cov.len(), 5)
    }

    #[test]
    fn test_coverage_diagonal_3() {
        let line = parser::lines_parser::line("1,5 -> 5,1").unwrap();
        let cov = line.coverage();
        assert_eq!(cov.len(), 5)
    }

    #[test]
    fn test_coverage_diagonal_4() {
        let line = parser::lines_parser::line("5,1 -> 1,5").unwrap();
        let cov = line.coverage();
        assert_eq!(cov.len(), 5)
    }
}
