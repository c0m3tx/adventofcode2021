use std::ops::RangeInclusive;

type Point = (i64, i64);

struct Probe {
    pos: Point,
    speed_x: i64,
    speed_y: i64,
}

impl Probe {
    fn new(speed_x: i64, speed_y: i64) -> Self {
        Probe {
            pos: (0, 0),
            speed_x,
            speed_y,
        }
    }

    fn fire(&mut self, target: &TargetArea) -> i32 {
        let mut steps = 0;
        loop {
            steps += 1;
            self.pos = (self.pos.0 + self.speed_x, self.pos.1 + self.speed_y);
            if self.speed_x > 0 {
                self.speed_x -= 1
            }
            self.speed_y -= 1;
            if self.pos.0 > *target.x.end() || self.pos.1 < *target.y.start() {
                return -1;
            }
            if target.includes(&self.pos) {
                return steps;
            }
        }
    }
}

struct TargetArea {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

impl TargetArea {
    fn includes(&self, point: &Point) -> bool {
        self.x.contains(&point.0) && self.y.contains(&point.1)
    }
}

fn input() -> TargetArea {
    TargetArea {
        x: 207..=263,
        y: -115..=-63,
    }
}

fn part_1(area: &TargetArea) -> i64 {
    let y = area.y.start();
    cumulative_sum(-y - 1)
}

fn cumulative_sum(speed: i64) -> i64 {
    (speed * (speed + 1)) / 2
}

fn part_2(area: &TargetArea) -> i64 {
    let mut count = 0;
    for x in 20..=263 {
        for y in -118..=6555 {
            if Probe::new(x, y).fire(area) > 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let target_area = input();
    part_1(&target_area);
    part_2(&target_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&input()), 6555);
    }

    #[test]
    fn test_part_2() {
        println!("{}", part_2(&input()));
    }

    #[test]
    fn test_fire() {
        let input = TargetArea {
            x: 20..=30,
            y: -10..=-5,
        };
        let mut probe = Probe::new(7, 2);
        assert_eq!(probe.fire(&input), 7);
    }
}
