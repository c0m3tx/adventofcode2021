use crate::line::{Line, Point};

peg::parser! {
    pub grammar lines_parser() for str {
        rule number() -> i64 = n:$(['0'..='9']+) { n.parse().unwrap() }
        rule point() -> Point = x:number() "," y:number() { Point::new(x,y) }
        pub rule line() -> Line = s:point() " -> " e:point() { Line::new(s,e) }
        pub rule file() -> Vec<Line> = lines:line() ** "\n" [_]* { lines }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = lines_parser::line("1,2 -> 3,4").expect("Unable to parse line");
        assert_eq!(line.start, Point::new(1, 2));
        assert_eq!(line.end, Point::new(3, 4));
    }

    #[test]
    fn test_parse() {
        let input = "964,133 -> 596,133\n920,215 -> 920,976\n123,528 -> 123,661\n613,13 -> 407,13\n373,876 -> 424,876\n616,326 -> 120,326\n486,335 -> 539,388\n";
        let result = lines_parser::file(input).expect("Unable to parse");
        assert_eq!(result.len(), 7);
        assert_eq!(result[0].start.x, 964);
    }

    #[test]
    fn test_parse_file() {
        let input = std::fs::read_to_string("input").unwrap();
        let result = lines_parser::file(&input).expect("Unable to parse");
        assert_eq!(result.len(), 500);
    }
}
