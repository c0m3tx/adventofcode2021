use crate::packet::Packet;

pub struct Operator {
    version: u8,
    subpackets: Vec<Box<dyn Packet>>,
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
    pub fn new(version: u8, subpackets: Vec<Box<dyn Packet>>) -> Self {
        Self {
            version,
            subpackets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::operator;

    #[test]
    fn test_operator_type_0() {
        let input = "00111000000000000110111101000101001010010001001000000000".to_owned();
        let (result, offset) = operator(&input, 0).unwrap();
        assert_eq!(result.version(), 1);
        assert_eq!(result.subpackets.len(), 2);
        assert_eq!(result.subpackets.get(0).unwrap().execute(), 10);
        assert_eq!(result.subpackets.get(1).unwrap().execute(), 20);
        assert_eq!(offset, 49);
    }

    #[test]
    fn test_operator_type_1() {
        let input = "11101110000000001101010000001100100000100011000001100000".to_owned();
        let (result, offset) = operator(&input, 0).unwrap();
        assert_eq!(result.version(), 7);
        assert_eq!(result.subpackets.len(), 3);
        assert_eq!(result.subpackets.get(0).unwrap().execute(), 1);
        assert_eq!(result.subpackets.get(1).unwrap().execute(), 2);
        assert_eq!(result.subpackets.get(2).unwrap().execute(), 3);
        assert_eq!(offset, 51);
    }
}
