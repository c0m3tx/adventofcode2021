fn binary_to_decimal(bin: &str) -> isize {
    isize::from_str_radix(bin, 2).unwrap()
}

fn filter_signals(signals: &Vec<String>, position: usize, bit: char) -> Vec<String> {
    signals
        .iter()
        .filter(|s| s.chars().nth(position).unwrap() == bit)
        .cloned()
        .collect()
}

fn count_bits(signals: &Vec<String>, position: usize) -> (usize, usize) {
    signals
        .iter()
        .map(|s| s.chars().nth(position).unwrap())
        .fold((0, 0), |acc, c| match c {
            '0' => (acc.0 + 1, acc.1),
            '1' => (acc.0, acc.1 + 1),
            _ => acc,
        })
}

fn energy_rates(signals: &Vec<String>) -> (isize, isize) {
    let signal_length = signals[0].len();
    let gamma: String = (0..signal_length)
        .map(|i| {
            let (zeroes, ones) = count_bits(&signals, i);
            if ones > zeroes {
                "1"
            } else {
                "0"
            }
        })
        .collect();

    let gamma = binary_to_decimal(&gamma);
    let epsilon = (1 << signal_length) - 1 - gamma;

    (gamma, epsilon)
}

fn oxygen_rating(signals: &Vec<String>, position: usize) -> isize {
    if signals.len() == 1 {
        return binary_to_decimal(&signals[0]);
    }

    let (zeroes, ones) = count_bits(signals, position);
    let signals = filter_signals(&signals, position, if ones >= zeroes { '1' } else { '0' });
    oxygen_rating(&signals, position + 1)
}

fn co2_scrubber_rating(signals: &Vec<String>, position: usize) -> isize {
    if signals.len() == 1 {
        return binary_to_decimal(&signals[0]);
    }

    let (zeroes, ones) = count_bits(signals, position);
    let signals = filter_signals(&signals, position, if ones >= zeroes { '0' } else { '1' });
    co2_scrubber_rating(&signals, position + 1)
}

fn main() {
    let content = std::fs::read_to_string("input").expect("Unable to read file");
    let signals: Vec<String> = content.lines().map(|line| line.into()).collect();

    let (gamma, epsilon) = energy_rates(&signals);
    let oxygen_rating = oxygen_rating(&signals, 0);
    let co2_scrubber_rating = co2_scrubber_rating(&signals, 0);

    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
    println!("Oxygen: {}", oxygen_rating);
    println!("CO2: {}", co2_scrubber_rating);
    println!("Life Support: {}", oxygen_rating * co2_scrubber_rating);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_signals() -> Vec<String> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|&s| s.into())
        .collect()
    }

    #[test]
    fn test_energy_rates() {
        let signals = test_signals();
        let (gamma_rate, epsilon_rate) = energy_rates(&signals);

        assert_eq!(gamma_rate, 22);
        assert_eq!(epsilon_rate, 9);
    }

    #[test]
    fn test_oxygen_rating() {
        let signals = test_signals();
        assert_eq!(oxygen_rating(&signals, 0), 23);
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let signals = test_signals();
        assert_eq!(co2_scrubber_rating(&signals, 0), 10);
    }
}
