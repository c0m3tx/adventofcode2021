#[derive(Debug)]
struct Pawn {
    position: u64,
    points: u64,
}

impl Pawn {
    fn new(position: u64) -> Self {
        Self {
            position: position - 1,
            points: 0,
        }
    }

    fn transpose(&mut self, amount: u64) {
        self.position += amount;
        self.position = self.position % 10;
        self.points += self.position + 1;
    }
}

struct DeterministicDie {
    next_value: u64,
    throws: u64,
}

impl DeterministicDie {
    fn new() -> Self {
        Self {
            next_value: 1,
            throws: 0,
        }
    }
}

impl Iterator for DeterministicDie {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.throws += 1;
        let value = self.next_value;
        self.next_value += 1;
        if self.next_value > 100 {
            self.next_value = 1;
        }

        Some(value)
    }
}

fn part_1(pos_1: u64, pos_2: u64, limit: u64) -> u64 {
    let dice = &mut DeterministicDie::new();
    let mut pawn_1 = Pawn::new(pos_1);
    let mut pawn_2 = Pawn::new(pos_2);

    loop {
        let dice_amount: u64 = dice.take(3).sum();
        pawn_1.transpose(dice_amount);

        if pawn_1.points >= limit {
            break;
        }

        let dice_amount: u64 = dice.take(3).sum();
        pawn_2.transpose(dice_amount);

        if pawn_2.points >= limit {
            break;
        }
    }

    pawn_1.points.min(pawn_2.points) * dice.throws
}

const FREQUENCIES: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn rec_solution(p1: u64, p2: u64, s1: u64, s2: u64, limit: u64) -> (u64, u64) {
    if s1 >= limit {
        return (1, 0);
    }
    if s2 >= limit {
        return (0, 1);
    }

    FREQUENCIES
        .iter()
        .fold((0, 0), |(p1_tw, p2_tw), (roll, frequency)| {
            let new_p1_position = (p1 + roll) % 10;
            let new_p1_score = s1 + new_p1_position + 1;

            let (p2_w, p1_w) = rec_solution(p2, new_p1_position, s2, new_p1_score, limit);

            (p1_tw + (frequency * p1_w), p2_tw + (frequency * p2_w))
        })
}

fn part_2(p1: u64, p2: u64, limit: u64) -> u64 {
    let (p1, p2) = rec_solution(p1 - 1, p2 - 1, 0, 0, limit);
    p1.max(p2)
}

fn main() {
    println!("{}", part_1(6, 7, 1000));
    println!("{}", part_2(6, 7, 21));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(4, 8, 1000), 739785)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(4, 8, 21), 444356092776315)
    }
}
