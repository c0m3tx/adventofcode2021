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
    let package_type = packet_type(&input, start);
    if let PacketType::Literal = package_type {
        return Err("Not an operator".to_string());
    }
    let version = range_u64(input, start..(start + 3)) as u8;

    if at(input, start + 6) == 0 {
        let length = range_u64(input, (start + 7)..(start + 22)) as usize;
        let input_subset = input[(start + 22)..(start + 22 + length)].to_string();
        let subpackets = load_all_subpackets(input_subset);
        let offset = start + length + 22;
        Ok((Operator::new(version, subpackets), offset))
    } else {
        let packets_count = range_u64(input, (start + 7)..(start + 18));
        let (subpackets, offset) = load_n_subpackets(input, start + 18, packets_count);
        Ok((Operator::new(version, subpackets), offset))
    }
}

fn load_all_subpackets(input: String) -> Vec<Box<dyn Packet>> {
    let mut subpackets: Vec<Box<dyn Packet>> = Vec::new();
    let mut position = 0;
    loop {
        let packet_type = packet_type(&input, position);

        match packet_type {
            PacketType::Literal => {
                let (literal, offset) = literal(&input, position).unwrap();
                subpackets.push(Box::new(literal));
                position = offset;
            }
            PacketType::Operator(_) => {
                let (operator, offset) = operator(&input, position).unwrap();
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

fn load_n_subpackets(input: &str, start: usize, count: u64) -> (Vec<Box<dyn Packet>>, usize) {
    let mut position = start;
    let mut subpackets: Vec<Box<dyn Packet>> = vec![];
    for _ in 0..count {
        let packet_type = packet_type(input, position);
        match packet_type {
            PacketType::Literal => {
                let (literal, offset) = literal(input, position).unwrap();
                subpackets.push(Box::new(literal));
                position = offset;
            }
            PacketType::Operator(_) => {
                let (operator, offset) = operator(input, position).unwrap();
                subpackets.push(Box::new(operator));
                position = offset;
            }
        }
    }

    (subpackets, position)
}
