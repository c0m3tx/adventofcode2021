fn main() {
    let input = parse_input();
    let total_explosions = part_1(input, 100);
    println!("Part 1: {}", total_explosions);
    let full_blast_iteration = part_2(input);
    println!("Part 2: {}", full_blast_iteration);
}

fn neighbors_coords((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (x + dx, y + dy)))
        .filter(|&(px, py)| px != x || py != y)
        .filter(|&(x, y)| x >= 0 && x < 10 && y >= 0 && y < 10)
        .collect()
}

fn index((x, y): (i32, i32)) -> usize {
    (y * 10 + x) as usize
}

fn coords(i: usize) -> (i32, i32) {
    ((i % 10).try_into().unwrap(), (i / 10).try_into().unwrap())
}

fn step(array: [i32; 100]) -> ([i32; 100], i32) {
    let mut array = array.clone();
    // increment all by one
    for i in 0..100 {
        array[i] += 1;
    }

    loop {
        let mut changed = false;
        for i in 0..100 {
            if array[i] > 9 && array[i] < 500 {
                // this should explode
                let co = coords(i);
                array[i] += 500;
                neighbors_coords(co).iter().for_each(|&(x, y)| {
                    array[index((x, y))] += 1;
                });
                changed = true;
            }
        }
        if !changed {
            // no more explosions!
            break;
        }
    }

    let mut explosions = 0;
    for i in 0..100 {
        if array[i] >= 500 {
            array[i] = 0;
            explosions += 1;
        }
    }

    (array, explosions)
}

fn part_1(input: [i32; 100], steps: i32) -> i32 {
    let mut input = input;
    let mut total_explosions = 0;
    for _ in 0..steps {
        let (result, explosions) = step(input);
        total_explosions += explosions;
        input = result;
    }

    total_explosions
}

fn part_2(input: [i32; 100]) -> i32 {
    let mut input = input;
    let mut iteration = 0;
    loop {
        iteration += 1;
        let (result, explosions) = step(input);
        if explosions == 100 {
            break;
        }
        input = result;
    }
    iteration
}

fn parse_input() -> [i32; 100] {
    let mut output = [0; 100];
    std::fs::read_to_string("input")
        .expect("Could not read input file")
        .lines()
        .flat_map(|line| line.trim().split("").map(|s| s.parse::<i32>()))
        .filter_map(|x| x.ok())
        .take(100)
        .enumerate()
        .for_each(|(i, n)| output[i] = n);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = parse_input();
        let expected = [
            4, 7, 6, 4, 7, 4, 5, 7, 8, 4, 4, 6, 4, 3, 4, 5, 7, 1, 7, 6, 8, 3, 2, 2, 6, 2, 8, 4, 7,
            7, 7, 6, 1, 7, 1, 5, 2, 5, 4, 6, 6, 1, 3, 7, 5, 1, 8, 1, 6, 5, 1, 5, 5, 6, 7, 2, 3, 1,
            7, 6, 2, 1, 8, 7, 8, 6, 1, 8, 8, 6, 2, 5, 5, 3, 4, 2, 2, 6, 2, 5, 4, 8, 1, 7, 5, 8, 4,
            6, 3, 8, 3, 7, 5, 4, 2, 8, 5, 6, 6, 2,
        ];

        assert_eq!(input, expected);
    }

    #[test]
    fn test_step_1() {
        let input = [
            5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
            3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6,
            4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8,
            5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
        ];

        let (result, explosions) = step(input);
        let expected = [
            6, 5, 9, 4, 2, 5, 4, 3, 3, 4, 3, 8, 5, 6, 9, 6, 5, 8, 2, 2, 6, 3, 7, 5, 6, 6, 7, 2, 8,
            4, 7, 2, 5, 2, 4, 4, 7, 2, 5, 7, 7, 4, 6, 8, 4, 9, 6, 5, 8, 9, 5, 2, 7, 8, 6, 3, 5, 7,
            5, 6, 3, 2, 8, 7, 9, 5, 2, 8, 3, 2, 7, 9, 9, 3, 9, 9, 2, 2, 4, 5, 5, 9, 5, 7, 9, 5, 9,
            6, 6, 5, 6, 3, 9, 4, 8, 6, 2, 6, 3, 7,
        ];
        assert_eq!(result, expected);
        assert_eq!(explosions, 0);
    }

    #[test]
    fn test_step_2() {
        let input = [
            6, 5, 9, 4, 2, 5, 4, 3, 3, 4, 3, 8, 5, 6, 9, 6, 5, 8, 2, 2, 6, 3, 7, 5, 6, 6, 7, 2, 8,
            4, 7, 2, 5, 2, 4, 4, 7, 2, 5, 7, 7, 4, 6, 8, 4, 9, 6, 5, 8, 9, 5, 2, 7, 8, 6, 3, 5, 7,
            5, 6, 3, 2, 8, 7, 9, 5, 2, 8, 3, 2, 7, 9, 9, 3, 9, 9, 2, 2, 4, 5, 5, 9, 5, 7, 9, 5, 9,
            6, 6, 5, 6, 3, 9, 4, 8, 6, 2, 6, 3, 7,
        ];
        let expected = [
            8, 8, 0, 7, 4, 7, 6, 5, 5, 5, 5, 0, 8, 9, 0, 8, 7, 0, 5, 4, 8, 5, 9, 7, 8, 8, 9, 6, 0,
            8, 8, 4, 8, 5, 7, 6, 9, 6, 0, 0, 8, 7, 0, 0, 9, 0, 8, 8, 0, 0, 6, 6, 0, 0, 0, 8, 8, 9,
            8, 9, 6, 8, 0, 0, 0, 0, 5, 9, 4, 3, 0, 0, 0, 0, 0, 0, 7, 4, 5, 6, 9, 0, 0, 0, 0, 0, 0,
            8, 7, 6, 8, 7, 0, 0, 0, 0, 6, 8, 4, 8,
        ];
        let (result, explosions) = step(input);
        assert_eq!(result, expected);
        assert_eq!(explosions, 35);
    }

    #[test]
    fn test_part_1() {
        let input = [
            5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
            3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6,
            4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8,
            5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
        ];

        let explosions = part_1(input, 100);
        assert_eq!(explosions, 1656);
    }

    #[test]
    fn test_part_2() {
        let input = [
            5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
            3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6,
            4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8,
            5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
        ];

        let iteration = part_2(input);
        assert_eq!(iteration, 195);
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(neighbors_coords((0, 0)).len(), 3);
        assert_eq!(neighbors_coords((9, 9)).len(), 3);
        assert_eq!(neighbors_coords((5, 9)).len(), 5);
        assert_eq!(neighbors_coords((5, 5)).len(), 8);
    }
}
