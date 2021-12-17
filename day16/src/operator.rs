use crate::packet::Packet;

#[derive(PartialEq, Debug)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
}

impl From<u64> for Operation {
    fn from(opcode: u64) -> Self {
        match opcode {
            0 => Operation::Sum,
            1 => Operation::Product,
            2 => Operation::Min,
            3 => Operation::Max,
            5 => Operation::GreaterThan,
            6 => Operation::LessThan,
            7 => Operation::Equal,
            _ => panic!("Unknown operation {}", opcode),
        }
    }
}

pub struct Operator {
    version: u8,
    operation: Operation,
    subpackets: Vec<Box<dyn Packet>>,
}

impl Packet for Operator {
    fn version_sum(&self) -> u64 {
        let subpacket_sum: u64 = self.subpackets.iter().map(|p| p.version_sum()).sum();
        self.version as u64 + subpacket_sum
    }

    fn execute(&self) -> u64 {
        match &self.operation {
            Operation::Sum => self.subpackets.iter().map(|sub| sub.execute()).sum(),
            Operation::Product => self.subpackets.iter().map(|sub| sub.execute()).product(),
            Operation::Min => self
                .subpackets
                .iter()
                .map(|sub| sub.execute())
                .min()
                .unwrap(),
            Operation::Max => self
                .subpackets
                .iter()
                .map(|sub| sub.execute())
                .max()
                .unwrap(),
            Operation::GreaterThan => {
                let first = self.subpackets[0].execute();
                let second = self.subpackets[1].execute();

                if first > second {
                    1
                } else {
                    0
                }
            }
            Operation::LessThan => {
                let first = self.subpackets[0].execute();
                let second = self.subpackets[1].execute();

                if first < second {
                    1
                } else {
                    0
                }
            }
            Operation::Equal => {
                let first = self.subpackets[0].execute();
                let second = self.subpackets[1].execute();

                if first == second {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl Operator {
    pub fn new(version: u8, opcode: u64, subpackets: Vec<Box<dyn Packet>>) -> Self {
        Self {
            version,
            operation: opcode.into(),
            subpackets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::literal::Literal;
    use crate::parsers::operator;

    #[test]
    fn test_operator_type_0() {
        let input = "00111000000000000110111101000101001010010001001000000000".to_owned();
        let (result, offset) = operator(&input, 0).unwrap();
        assert_eq!(result.version, 1);
        assert_eq!(result.operation, Operation::LessThan);
        assert_eq!(result.subpackets.len(), 2);
        assert_eq!(result.subpackets.get(0).unwrap().execute(), 10);
        assert_eq!(result.subpackets.get(1).unwrap().execute(), 20);
        assert_eq!(offset, 49);
    }

    #[test]
    fn test_operator_type_1() {
        let input = "11101110000000001101010000001100100000100011000001100000".to_owned();
        let (result, offset) = operator(&input, 0).unwrap();
        assert_eq!(result.version, 7);
        assert_eq!(result.operation, Operation::Max);
        assert_eq!(result.subpackets.len(), 3);
        assert_eq!(result.subpackets.get(0).unwrap().execute(), 1);
        assert_eq!(result.subpackets.get(1).unwrap().execute(), 2);
        assert_eq!(result.subpackets.get(2).unwrap().execute(), 3);
        assert_eq!(offset, 51);
    }

    macro_rules! operator {
        ($opcode:expr, [$($subpackets:expr),*]) => {
            Operator::new(1, $opcode, vec![$(Box::new($subpackets)),*])
        };
    }

    #[test]
    fn test_operator_sum() {
        let operator = operator!(0, [Literal::new(1, 1), Literal::new(1, 2)]);
        assert_eq!(operator.execute(), 3);
    }

    #[test]
    fn test_operator_product() {
        let operator = operator!(1, [Literal::new(1, 2), Literal::new(1, 2)]);
        assert_eq!(operator.execute(), 4);
    }

    #[test]
    fn test_operator_min() {
        let operator = operator!(2, [Literal::new(1, 1), Literal::new(1, 2)]);
        assert_eq!(operator.execute(), 1);
    }

    #[test]
    fn test_operator_max() {
        let operator = operator!(3, [Literal::new(1, 1), Literal::new(1, 2)]);
        assert_eq!(operator.execute(), 2);
    }

    #[test]
    fn test_operator_greater_than() {
        let operator = operator!(5, [Literal::new(1, 1), Literal::new(1, 2)]);
        assert_eq!(operator.execute(), 0);

        let operator = operator!(5, [Literal::new(1, 2), Literal::new(1, 1)]);
        assert_eq!(operator.execute(), 1);
    }

    #[test]
    fn test_operator_less_than() {
        let operator = operator!(6, [Literal::new(1, 1), Literal::new(1, 2)]);
        assert_eq!(operator.execute(), 1);

        let operator = operator!(6, [Literal::new(1, 2), Literal::new(1, 1)]);
        assert_eq!(operator.execute(), 0);
    }

    #[test]
    fn test_operator_equal() {
        let operator = operator!(7, [Literal::new(1, 1), Literal::new(1, 2)]);
        assert_eq!(operator.execute(), 0);

        let operator = operator!(7, [Literal::new(1, 1), Literal::new(1, 1)]);
        assert_eq!(operator.execute(), 1);
    }
}
