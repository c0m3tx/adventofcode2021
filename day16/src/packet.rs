use crate::utils::range_u64;

pub enum PacketType {
    Literal,
    Operator(u64),
}

pub trait Packet {
    fn version_sum(&self) -> u64;
    fn execute(&self) -> u64;
}

pub fn packet_type(input: &str, start: usize) -> PacketType {
    match range_u64(&input, (start + 3)..(start + 6)) {
        4 => PacketType::Literal,
        x => PacketType::Operator(x),
    }
}
