mod parser;

struct Submarine {
    depth: i64,
    distance: i64,
    aim: i64,
}

fn coarse_calculation(cmds: &Vec<parser::Command>) -> i64 {
    let mut s = Submarine {
        depth: 0,
        distance: 0,
        aim: 0,
    };

    cmds.iter().for_each(|cmd| match cmd {
        parser::Command::Up(d) => s.depth -= d,
        parser::Command::Down(d) => s.depth += d,
        parser::Command::Forward(d) => s.distance += d,
    });

    s.depth * s.distance
}

fn accurate_calculation(cmds: &Vec<parser::Command>) -> i64 {
    let mut s = Submarine {
        depth: 0,
        distance: 0,
        aim: 0,
    };

    cmds.iter().for_each(|cmd| match cmd {
        parser::Command::Up(x) => {
            s.aim -= *x;
        }
        parser::Command::Down(x) => {
            s.aim += *x;
        }
        parser::Command::Forward(x) => {
            s.depth = s.depth + (s.aim * x);
            s.distance += x;
        }
    });

    s.depth * s.distance
}

fn main() {
    let content = std::fs::read_to_string("input").expect("Unable to read file");
    let result = parser::parse(&content).expect("Unable to parse file");
    println!("Coarse movement: {}", coarse_calculation(&result));
    println!("Accurate movement: {}", accurate_calculation(&result));
}
