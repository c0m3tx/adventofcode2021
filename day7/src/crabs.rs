use std::ops::RangeInclusive;

pub fn parse(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|x| {
            x.trim()
                .parse::<i64>()
                .expect(format!("Unable to parse '{}'", x).as_str())
        })
        .collect()
}

fn linear_fuel_for_position(crabs: &Vec<i64>, position: i64) -> i64 {
    crabs.iter().fold(0, |acc, &c| (c - position).abs() + acc)
}

fn incremental_fuel_for_position(crabs: &Vec<i64>, position: i64) -> i64 {
    crabs.iter().fold(0, |acc, &c| {
        let dist = (c - position).abs();
        let cons = (dist * (dist + 1)) / 2;
        acc + cons
    })
}

pub fn find_best_linear_fuel_consumption(crabs: &Vec<i64>) -> i64 {
    range(crabs)
        .map(|x| linear_fuel_for_position(crabs, x))
        .min()
        .unwrap()
}

pub fn find_best_incremental_fuel_consumption(crabs: &Vec<i64>) -> i64 {
    range(crabs)
        .map(|x| incremental_fuel_for_position(crabs, x))
        .min()
        .unwrap()
}

fn range(crabs: &Vec<i64>) -> RangeInclusive<i64> {
    let (min, max) = crabs.iter().fold((i64::MAX, i64::MIN), |(min, max), &c| {
        (c.min(min), c.max(max))
    });

    min..=max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_fuel_for_position() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let fuel = linear_fuel_for_position(&crabs, 2);
        assert_eq!(fuel, 37)
    }

    #[test]
    fn test_incremental_fuel_for_position() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let fuel = incremental_fuel_for_position(&crabs, 2);
        assert_eq!(fuel, 206)
    }

    #[test]
    fn test_range() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let range = range(&crabs);
        assert_eq!(range, 0..=16)
    }

    #[test]
    fn test_find_best_fuel_consumption() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let best_fuel = find_best_linear_fuel_consumption(&crabs);
        assert_eq!(best_fuel, 37);
    }
}
