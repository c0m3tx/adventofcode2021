use std::fs;

fn load_input() -> Vec<u64> {
    fs::read_to_string("input")
        .expect("Unable to read file")
        .lines()
        .map(|l| {
            l.parse::<u64>()
                .expect(format!("Unable to parse line {}", l).as_str())
        })
        .collect()
}

fn increments(input: &Vec<u64>) -> usize {
    input.windows(2).filter(|win| win[0] < win[1]).count()
}

fn window_increments(input: &Vec<u64>) -> usize {
    let values = input
        .windows(3)
        .map(|win| win.iter().sum())
        .collect::<Vec<u64>>();
    increments(&values)
}

fn main() {
    let measurements: Vec<u64> = load_input();
    let incr = window_increments(&measurements);

    println!("Increments: {}", incr);
}

#[cfg(test)]
mod tests {

    #[test]
    fn load_input() {
        let input = super::load_input();
        assert_eq!(input[0], 134);
        assert_eq!(input[1], 138);
        assert_eq!(input[2], 142);
        assert_eq!(input.into_iter().rev().next(), Some(10753u64))
    }

    #[test]
    fn increments() {
        let input: Vec<u64> = vec![1, 10, 50, 30];
        let increments = super::increments(&input);
        assert_eq!(increments, 2);
    }

    #[test]
    fn window_increments() {
        let input: Vec<u64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let window_increments = super::window_increments(&input);
        assert_eq!(window_increments, 5)
    }
}
