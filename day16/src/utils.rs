use std::ops::Range;

pub fn from_hex(hex: &str) -> String {
    hex.chars()
        .map(|x| x.to_digit(16).unwrap() as u8)
        .map(|x| format!("{:04b}", x))
        .collect::<String>()
}

pub fn range_u64(bits: &str, range: Range<usize>) -> u64 {
    let mut value: u64 = 0;
    range.rev().enumerate().for_each(|(i, pos)| {
        value += (at(bits, pos) as u64) << i;
    });

    value
}

pub fn string_to_u64(s: &str) -> u64 {
    let len = s.len();
    let mut result = 0;
    for (i, c) in s.chars().enumerate() {
        result += (c.to_digit(10).unwrap() as u64) << (len - i - 1);
    }

    result
}

pub fn at(bits: &str, pos: usize) -> u8 {
    bits.chars()
        .nth(pos)
        .and_then(|f| f.to_digit(2).map(|d| d as u8))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_convertion() {
        let output = from_hex("D2FE28");
        assert_eq!(output, "110100101111111000101000".to_owned())
    }

    #[test]
    fn test_string_to_u64() {
        let result = string_to_u64("10001");
        assert_eq!(result, 17)
    }
}
