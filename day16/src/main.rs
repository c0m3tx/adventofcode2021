use std::ops::Range;

#[derive(PartialEq, Debug)]
struct Literal {
    version: u8,
    value: u64,
}

struct Operator {
    version: u8,
    subpackets: Vec<Box<dyn Packet>>,
}

enum PacketType {
    Literal,
    Operator(u64),
}

trait Packet {
    fn version(&self) -> u64;
    fn version_sum(&self) -> u64;
    fn execute(&self) -> u64;
}

impl Packet for Literal {
    fn version(&self) -> u64 {
        self.version as u64
    }

    fn version_sum(&self) -> u64 {
        self.version()
    }

    fn execute(&self) -> u64 {
        self.value
    }
}

impl Packet for Operator {
    fn version(&self) -> u64 {
        self.version as u64
    }

    fn version_sum(&self) -> u64 {
        let subpacket_sum: u64 = self.subpackets.iter().map(|p| p.version_sum()).sum();
        self.version() + subpacket_sum
    }

    fn execute(&self) -> u64 {
        panic!("AAAAAAAAAAAAAAAAAAAAA")
    }
}

impl Operator {
    fn new(version: u8, subpackets: Vec<Box<dyn Packet>>) -> Self {
        Self {
            version,
            subpackets,
        }
    }
}

struct Bits {
    bits: String,
}

impl From<String> for Bits {
    fn from(hex: String) -> Self {
        let bits = hex
            .chars()
            .map(|x| x.to_digit(16).unwrap() as u8)
            .map(|x| format!("{:04b}", x))
            .collect::<String>();
        Bits { bits }
    }
}

fn string_to_u64(s: &str) -> u64 {
    let len = s.len();
    let mut result = 0;
    for (i, c) in s.chars().enumerate() {
        result += (c.to_digit(10).unwrap() as u64) << (len - i - 1);
    }

    result
}

impl Bits {
    fn at(&self, pos: usize) -> u8 {
        match self.bits.chars().nth(pos) {
            Some('0') => 0,
            Some('1') => 1,
            _ => panic!("Not a number?"),
        }
    }

    fn range_u64(&self, range: Range<usize>) -> u64 {
        let mut value: u64 = 0;
        range.rev().enumerate().for_each(|(i, pos)| {
            value += (self.at(pos) as u64) << i;
        });

        value
    }
}

const OPERATOR: u64 = 0;

fn packet_type(input: &Bits, start: usize) -> PacketType {
    match input.range_u64((start + 3)..(start + 6)) {
        4 => PacketType::Literal,
        x => PacketType::Operator(x),
    }
}

fn literal(input: &Bits, start: usize) -> Result<(Literal, usize), String> {
    let package_type = input.range_u64((start + 3)..(start + 6));
    if package_type != 4 {
        return Err("Not a literal".to_string());
    }
    let version = input.range_u64(start..(start + 3)) as u8;
    let mut position = start + 6;
    let mut output = String::default();
    loop {
        for i in 1..5 {
            output.push(input.bits.chars().nth(position + i).unwrap());
        }

        if input.at(position) == 0 {
            break;
        }
        position += 5;
    }
    let result = Literal {
        value: string_to_u64(&output),
        version,
    };
    Ok((result, position + 5))
}

fn operator(input: &Bits, start: usize) -> Result<(Operator, usize), String> {
    let package_type = packet_type(&input, start);
    if let PacketType::Literal = package_type {
        return Err("Not an operator".to_string());
    }
    let version = input.range_u64(start..(start + 3)) as u8;

    if input.at(start + 6) == 0 {
        let length = input.range_u64((start + 7)..(start + 22)) as usize;
        let subset = input.bits[(start + 22)..(start + 22 + length)].to_string();
        let subpackets = load_all_subpackets(subset);
        let offset = start + length + 22;
        Ok((Operator::new(version, subpackets), offset))
    } else {
        let packets_count = input.range_u64((start + 7)..(start + 18));
        let (subpackets, offset) = load_n_subpackets(&input, start + 18, packets_count);
        Ok((Operator::new(version, subpackets), offset))
    }
}

fn load_all_subpackets(input: String) -> Vec<Box<dyn Packet>> {
    let bits = Bits {
        bits: input.clone(),
    };
    let mut subpackets: Vec<Box<dyn Packet>> = Vec::new();
    let mut position = 0;
    loop {
        let packet_type = packet_type(&bits, position);

        match packet_type {
            PacketType::Literal => {
                let (literal, offset) = literal(&bits, position).unwrap();
                subpackets.push(Box::new(literal));
                position = offset;
            }
            PacketType::Operator(_) => {
                let (operator, offset) = operator(&bits, position).unwrap();
                subpackets.push(Box::new(operator));
                position = offset;
            }
        }
        if input.len() - position <= 8 {
            break;
        }
    }

    subpackets
}

fn load_n_subpackets(bits: &Bits, start: usize, count: u64) -> (Vec<Box<dyn Packet>>, usize) {
    let mut position = start;
    let mut subpackets: Vec<Box<dyn Packet>> = vec![];
    for _ in 0..count {
        let packet_type = packet_type(&bits, position);
        match packet_type {
            PacketType::Literal => {
                let (literal, offset) = literal(&bits, position).unwrap();
                subpackets.push(Box::new(literal));
                position = offset;
            }
            PacketType::Operator(_) => {
                let (operator, offset) = operator(&bits, position).unwrap();
                subpackets.push(Box::new(operator));
                position = offset;
            }
        }
    }

    (subpackets, position)
}

fn main() {
    part_1();
}

fn part_1() -> Box<dyn Packet> {
    let input = std::fs::read_to_string("input").expect("Unable to read input");
    let binary = Bits::from(input);
    match packet_type(&binary, 0) {
        PacketType::Literal => {
            let (packet, _) = literal(&binary, 0).unwrap();

            Box::from(packet)
        }
        PacketType::Operator(_) => {
            let (packet, _) = operator(&binary, 0).unwrap();

            Box::from(packet)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_convertion() {
        let output = Bits::from("D2FE28".to_owned());
        assert_eq!(output.bits, "110100101111111000101000".to_owned())
    }

    #[test]
    fn test_literal() {
        let input = Bits {
            bits: "110100101111111000101000".to_owned(),
        };
        let (result, offset) = literal(&input, 0).unwrap();
        assert_eq!(
            result,
            Literal {
                version: 0b110,
                value: 2021
            }
        );
        assert_eq!(offset, 21);
    }

    #[test]
    fn test_operator_type_0() {
        let input = Bits {
            bits: "00111000000000000110111101000101001010010001001000000000".to_owned(),
        };
        let (result, offset) = operator(&input, 0).unwrap();
        assert_eq!(result.version(), 1);
        assert_eq!(result.subpackets.len(), 2);
        assert_eq!(result.subpackets.get(0).unwrap().execute(), 10);
        assert_eq!(result.subpackets.get(1).unwrap().execute(), 20);
        assert_eq!(offset, 49);
    }

    #[test]
    fn test_operator_type_1() {
        let input = Bits {
            bits: "11101110000000001101010000001100100000100011000001100000".to_owned(),
        };
        let (result, offset) = operator(&input, 0).unwrap();
        assert_eq!(result.version(), 7);
        assert_eq!(result.subpackets.len(), 3);
        assert_eq!(result.subpackets.get(0).unwrap().execute(), 1);
        assert_eq!(result.subpackets.get(1).unwrap().execute(), 2);
        assert_eq!(result.subpackets.get(2).unwrap().execute(), 3);
        assert_eq!(offset, 51);
    }

    #[test]
    fn test_string_to_u64() {
        let result = string_to_u64("10001");
        assert_eq!(result, 17)
    }

    #[test]
    fn test_part_1() {
        let packet = part_1();
        assert_eq!(packet.version_sum(), 10);
    }
}
