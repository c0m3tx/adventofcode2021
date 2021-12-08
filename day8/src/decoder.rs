use crate::input::Input;
use crate::signal;

pub fn digits(input: &Input) -> i64 {
    let assoc = associate_numbers(&input.patterns);
    input
        .digits
        .iter()
        .map(|digit| {
            assoc
                .iter()
                .position(|e| signal::matches(e, digit))
                .unwrap()
                .to_string()
        })
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

pub fn associate_numbers(input: &Vec<String>) -> [String; 10] {
    let mut out: [String; 10] = Default::default();

    // "one" is the only 2-length pattern
    out[1] = signal::for_length(input, 2);
    // "four" is the only 4-length pattern
    out[4] = signal::for_length(input, 4);
    // "seven" is the only 3-length pattern
    out[7] = signal::for_length(input, 3);
    // "eight" is the only 7-length pattern
    out[8] = signal::for_length(input, 7);
    // "three" is the intersection of any pair of 5-length patterns (2, 3 or 5), merged with a 1
    out[3] = three(input, &out[1]);
    // "six" is a 6-length pattern which does not include the 1 pattern
    out[6] = six(input, &out[1]);
    // "nine" is a 6-length pattern which includes 1 and 9 patterns
    out[9] = nine(input, &out[1], &out[3]);
    // "zero" is a 6-length pattern which includes 1 pattern but not 3
    out[0] = zero(input, &out[1], &out[3]);
    // "five" is a 5-length pattern which is included in 9 and is not 3
    out[5] = five(input, &out[3], &out[9]);
    // "two" is a 5-length pattern which is neither a 3 nor a 5
    out[2] = two(input, &out[3], &out[5]);

    out
}

fn three(patterns: &Vec<String>, one: &str) -> String {
    let five_length_signals: Vec<&String> = patterns.iter().filter(|pat| pat.len() == 5).collect();

    let three = signal::union(
        one,
        &signal::intersection(
            five_length_signals[0].as_str(),
            five_length_signals[1].as_str(),
        ),
    );
    signal::find(&three, patterns)
}

fn six(patterns: &Vec<String>, one: &str) -> String {
    let six = patterns
        .iter()
        .find(|pat| pat.len() == 6 && !signal::contains(pat, one))
        .unwrap();

    signal::find(&six, patterns)
}

fn nine(patterns: &Vec<String>, one: &str, three: &str) -> String {
    let nine = patterns
        .iter()
        .find(|pat| pat.len() == 6 && signal::contains(pat, one) && signal::contains(pat, three))
        .unwrap();

    signal::find(&nine, patterns)
}

fn zero(patterns: &Vec<String>, one: &str, three: &str) -> String {
    let zero = patterns
        .iter()
        .find(|pat| pat.len() == 6 && signal::contains(pat, one) && !signal::contains(pat, three))
        .unwrap();

    signal::find(&zero, patterns)
}

fn five(patterns: &Vec<String>, three: &str, nine: &str) -> String {
    let five = patterns
        .iter()
        .find(|pat| pat.len() == 5 && signal::contains(nine, pat) && !signal::matches(pat, three))
        .unwrap();

    signal::find(&five, patterns)
}

fn two(patterns: &Vec<String>, three: &str, five: &str) -> String {
    let two = patterns
        .iter()
        .find(|pat| pat.len() == 5 && !signal::matches(pat, &three) && !signal::matches(pat, &five))
        .unwrap();

    signal::find(&two, patterns)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Input;

    #[test]
    fn test_three() {
        let input = Input::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let result = three(&input.patterns, "ab");
        assert_eq!(result, "fbcad")
    }

    #[test]
    fn test_nine() {
        let input = Input::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let result = nine(&input.patterns, "ab", "fbcad");
        assert_eq!(result, "cefabd")
    }

    #[test]
    fn test_five() {
        let input = Input::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let result = five(&input.patterns, "fbcad", "cefabd");
        assert_eq!(result, "cdfbe")
    }

    #[test]
    fn test_six() {
        let input = Input::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let result = six(&input.patterns, "ab");
        assert_eq!(result, "cdfgeb")
    }

    #[test]
    fn test_zero() {
        let input = Input::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let result = zero(&input.patterns, "ab", "fbcad");
        assert_eq!(result, "cagedb")
    }

    #[test]
    fn test_two() {
        let input = Input::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let result = two(&input.patterns, "fbcad", "cdfbe");
        assert_eq!(result, "gcdfa")
    }

    #[test]
    fn test_associate_numbers() {
        let input = Input::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let result = associate_numbers(&input.patterns);
        assert_eq!(
            result,
            [
                "cagedb", "ab", "gcdfa", "fbcad", "eafb", "cdfbe", "cdfgeb", "dab", "acedgfb",
                "cefabd"
            ]
        );
    }

    #[test]
    fn test_digits() {
        let input = Input::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let result = digits(&input);
        assert_eq!(result, 5353)
    }
}
