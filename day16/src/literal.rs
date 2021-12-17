use crate::packet::Packet;

#[derive(PartialEq, Debug)]
pub struct Literal {
    version: u8,
    value: u64,
}

impl Literal {
    pub fn new(version: u8, value: u64) -> Self {
        Literal { version, value }
    }
}

impl Packet for Literal {
    fn version_sum(&self) -> u64 {
        self.version as u64
    }

    fn execute(&self) -> u64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::literal;
    #[test]
    fn test_literal() {
        let input = "110100101111111000101000".to_owned();
        let (result, offset) = literal(&input, 0).unwrap();
        assert_eq!(result.version, 0b110);
        assert_eq!(result.value, 2021);
        assert_eq!(offset, 21);
    }
}
