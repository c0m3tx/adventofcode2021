mod board;
use board::Board;

fn main() {
    first_winner();
    last_winner();
}

fn first_winner() {
    let (extractions, mut boards) = load_boards_and_inputs();
    for e in extractions {
        for board in &mut boards {
            let found = board.extract(e);
            if found && board.winning() {
                println!(
                    "First winner score: {}\n{}\n",
                    board.calculate_score() * e,
                    board
                );
                return;
            }
        }
    }
}

fn last_winner() {
    let mut last_winner = 0;
    let mut last_board: Option<usize> = None;

    let (extractions, mut boards) = load_boards_and_inputs();
    for e in extractions {
        boards.iter_mut().enumerate().for_each(|(idx, board)| {
            let found = board.extract(e);
            if found && !board.already_won && board.winning() {
                board.already_won = true;
                last_winner = board.calculate_score() * e;
                last_board = Some(idx);
            }
        });
    }

    let last_board = &boards[last_board.unwrap()];
    println!("Last winner score: {}\n{}\n", last_winner, last_board);
}

fn load_boards_and_inputs() -> (Vec<u64>, Vec<Board>) {
    let input_file = std::fs::read_to_string("input").expect("Unable to read input file");
    let mut lines = input_file.lines();
    let extractions: Vec<u64> = lines
        .next()
        .expect("No lines in file?")
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    lines.next();
    let mut boards: Vec<Board> = vec![];

    let mut current_board: Vec<u64> = vec![];
    while let Some(line) = lines.next() {
        if line == "" {
            boards.push(Board::from(current_board));
            current_board = vec![];
        } else {
            line.split_whitespace().for_each(|x| {
                let cell = x.parse::<u64>().unwrap();
                current_board.push(cell);
            });
        }
    }

    if current_board != vec![] {
        boards.push(Board::from(current_board));
    }

    (extractions, boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_boards_and_inputs() {
        let (extractions, boards) = load_boards_and_inputs();
        assert_eq!(extractions[0], 15);

        assert_eq!(boards[0].at(0, 0).value, 26);
        assert_eq!(boards.len(), 100);
        assert_eq!(boards[99].at(0, 0).value, 67);
    }
}
