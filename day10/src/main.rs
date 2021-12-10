#[derive(PartialEq, Debug)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angled,
}

impl Bracket {
    fn corruption_value(&self) -> u64 {
        use Bracket::*;
        match self {
            Round => 3,
            Square => 57,
            Curly => 1197,
            Angled => 25137,
        }
    }

    fn completion_value(&self) -> u64 {
        use Bracket::*;
        match self {
            Round => 1,
            Square => 2,
            Curly => 3,
            Angled => 4,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| match parse_line(line) {
        Err(error) => acc + error.corruption_value(),
        Ok(_) => acc,
    })
}

fn part_2(input: &str) -> u64 {
    let mut values = input
        .lines()
        .map(|line| match parse_line(line) {
            Err(_) => None,
            Ok(stack) => stack
                .iter()
                .rev()
                .fold(0, |acc, bracket| (acc * 5) + bracket.completion_value())
                .into(),
        })
        .filter_map(|v| v)
        .collect::<Vec<u64>>();

    values.sort();
    values[values.len() / 2]
}

fn parse_line(input: &str) -> Result<Vec<Bracket>, Bracket> {
    use Bracket::*;
    let mut stack = vec![];

    macro_rules! op {
        (open $bracket:expr) => {
            stack.push($bracket)
        };
        (close $bracket:expr) => {
            if stack.pop() != Some($bracket) {
                return Err($bracket);
            }
        };
    }

    for c in input.chars() {
        match c {
            '(' => op!(open Round),
            '[' => op!(open Square),
            '{' => op!(open Curly),
            '<' => op!(open Angled),
            ')' => op!(close Round),
            ']' => op!(close Square),
            '}' => op!(close Curly),
            '>' => op!(close Angled),
            _ => panic!("Unexpected character {}", c),
        }
    }

    Ok(stack)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_returns_error_if_found() {
        let input = "{([(<{}[<>[]}>{[]{[(<()>";
        assert_eq!(parse_line(input), Err(Bracket::Curly));
    }

    #[test]
    fn parse_returns_stack_if_no_error() {
        use super::Bracket::*;
        // What remains: [({([[{{
        let input = "[({(<(())[]>[[{[]{<()<>>";
        assert_eq!(
            parse_line(input),
            // This is obviously reversed, must be read right-to-left (pop from stack, or iter().rev())
            Ok(vec![
                Square, Round, Curly, Round, Square, Square, Curly, Curly
            ])
        );
    }

    #[test]
    fn test_part_1() {
        let input = "{([(<{}[<>[]}>{[]{[(<()>\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{";

        assert_eq!(part_1(input), 26397);
    }

    #[test]
    fn test_part_2() {
        let input = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n(((({<>}<{<{<>}{[]{[]{}\n{<[[]]>}<{[{[{[]{()[[[]\n<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(part_2(input), 288957);
    }
}
