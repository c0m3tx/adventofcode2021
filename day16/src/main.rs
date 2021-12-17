mod literal;
mod operator;
mod packet;
mod parsers;
mod utils;
use packet::*;
use utils::*;

fn main() {
    let packet = parse();
    println!("Part 1: {:?}", packet.version_sum());
    println!("Part 1: {:?}", packet.execute());
}

fn parse() -> Box<dyn Packet> {
    let input = std::fs::read_to_string("input").expect("Unable to read input");
    let binary = from_hex(&input);
    match packet_type(&binary, 0) {
        PacketType::Literal => Box::from(parsers::literal(&binary, 0).unwrap().0),
        PacketType::Operator(_) => Box::from(parsers::operator(&binary, 0).unwrap().0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let packet = parse();
        assert_eq!(packet.version_sum(), 886);
    }

    #[test]
    fn test_part_2() {
        let packet = parse();
        assert_eq!(packet.execute(), 184487454837);
    }
}
