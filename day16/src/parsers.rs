use crate::literal::Literal;
use crate::operator::Operator;
use crate::packet::packet_type;
use crate::packet::Packet;
use crate::packet::PacketType;
use crate::utils::*;

pub fn literal(input: &str, start: usize) -> Result<(Literal, usize), String> {
    let package_type = range_u64(input, (start + 3)..(start + 6));
    if package_type != 4 {
        return Err("Not a literal".to_string());
    }
    let version = range_u64(input, start..(start + 3)) as u8;
    let mut position = start + 6;
    let mut output = String::default();
    loop {
        for i in 1..5 {
            output.push(input.chars().nth(position + i).unwrap());
        }

        if at(input, position) == 0 {
            break;
        }
        position += 5;
    }
    let result = Literal::new(version, string_to_u64(&output));
    Ok((result, position + 5))
}

pub fn operator(input: &str, start: usize) -> Result<(Operator, usize), String> {
    if let PacketType::Operator(opcode) = packet_type(&input, start) {
        let version = range_u64(input, start..(start + 3)) as u8;

        if at(input, start + 6) == 0 {
            let length = range_u64(input, (start + 7)..(start + 22)) as usize;
            let input_subset = input[(start + 22)..(start + 22 + length)].to_string();
            let subpackets = load_all_subpackets(input_subset);
            let offset = start + length + 22;
            Ok((Operator::new(version, opcode, subpackets), offset))
        } else {
            let packets_count = range_u64(input, (start + 7)..(start + 18));
            let (subpackets, offset) = load_n_subpackets(input, start + 18, packets_count);
            Ok((Operator::new(version, opcode, subpackets), offset))
        }
    } else {
        return Err("Not an operator".to_string());
    }
}

fn parse_subpacket(input: &str, start: usize) -> (Box<dyn Packet>, usize) {
    let packet_type = packet_type(&input, start);

    match packet_type {
        PacketType::Literal => {
            let (literal, offset) = literal(input, start).unwrap();
            let packet = Box::new(literal);

            (packet, offset)
        }
        PacketType::Operator(_) => {
            let (operator, offset) = operator(&input, start).unwrap();
            let packet = Box::new(operator);

            (packet, offset)
        }
    }
}

fn load_all_subpackets(input: String) -> Vec<Box<dyn Packet>> {
    let mut subpackets: Vec<Box<dyn Packet>> = Vec::new();
    let mut position = 0;
    while position < input.len() {
        let (packet, new_position) = parse_subpacket(&input, position);
        subpackets.push(packet);
        position = new_position;
    }

    subpackets
}

fn load_n_subpackets(input: &str, start: usize, count: u64) -> (Vec<Box<dyn Packet>>, usize) {
    let mut subpackets: Vec<Box<dyn Packet>> = vec![];
    let mut position = start;
    for _ in 0..count {
        let (packet, new_position) = parse_subpacket(&input, position);
        subpackets.push(packet);
        position = new_position;
    }

    (subpackets, position)
}
