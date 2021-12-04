use peg::error::ParseError;
use peg::str::LineCol;

#[derive(PartialEq, Debug)]
pub enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

pub fn parse(content: &str) -> Result<Vec<Command>, ParseError<LineCol>> {
    peg::parser! {
        grammar command_list_parser() for str {
            rule number() -> i64 = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule forward() -> Command = "forward " n:number() { Command::Forward(n) }
            rule up() -> Command = "up " n:number() { Command::Up(n) }
            rule down() -> Command = "down " n:number() { Command::Down(n) }
            rule command() -> Command = forward() / down() / up()

            pub rule root() -> Vec<Command> = c:command() ** "\n" [_]* { c }
        }
    };

    command_list_parser::root(content)
}

#[cfg(test)]
mod tests {
    use super::Command::*;
    use super::*;

    #[test]
    fn test_command_parsing() {
        let content = "forward 6
forward 9
down 9
up 7";

        let result = parse(content).expect("Unable to parse commands");
        assert_eq!(result, vec![Forward(6), Forward(9), Down(9), Up(7)]);
    }

    #[test]
    fn test_parse_file() {
        let content = std::fs::read_to_string("input").expect("Unable to read file");
        let result = parse(&content).expect("Unable to parse file");
        assert_eq!(result.len(), 1000);
        assert_eq!(result[0], Forward(6));
        assert_eq!(result[11], Down(2));
        assert_eq!(result[17], Up(7));
        assert_eq!(result[999], Forward(5));
    }
}
