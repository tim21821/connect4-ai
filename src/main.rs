use std::cmp;
use std::fs;
use std::io::stdin;
use std::iter::zip;
use std::time::Instant;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Clone)]
struct Position {
    board: [[i8; WIDTH]; HEIGHT],
    height: [usize; WIDTH],
    num_moves: i8,
    current: i8,
}

impl Position {
    fn new() -> Self {
        return Self {
            board: [[0; WIDTH]; HEIGHT],
            height: [0; WIDTH],
            num_moves: 0,
            current: 1,
        };
    }

    fn from_sequence(seq: &str) -> Self {
        let mut p = Self::new();
        for c in seq.chars() {
            let col: usize = (c.to_digit(10).unwrap() - 1) as usize;
            p.play(col);
        }
        return p;
    }

    fn negamax(&self) -> i8 {
        if self.num_moves == (WIDTH * HEIGHT) as i8 {
            return 0;
        }

        for col in 0..WIDTH {
            if self.can_play(col) && self.is_winning_move(col) {
                return ((WIDTH * HEIGHT + 1) as i8 - self.num_moves) / 2;
            }
        }

        let mut best_score: i8 = i8::MIN;

        for col in 0..WIDTH {
            if self.can_play(col) {
                let mut new_position = self.clone();
                new_position.play(col);
                let score = -1 * new_position.negamax();
                best_score = cmp::max(best_score, score);
            }
        }
        return best_score;
    }

    fn play(&mut self, col: usize) {
        self.board[self.height[col]][col] = self.current;
        self.height[col] += 1;
        self.num_moves += 1;
        self.current *= -1;
    }

    fn can_play(&self, col: usize) -> bool {
        return self.height[col] < HEIGHT;
    }

    fn is_winning_move(&self, col: usize) -> bool {
        return self.check_vertical(col) || self.check_horizontal(col) || self.check_diagonal(col);
    }

    fn check_vertical(&self, col: usize) -> bool {
        return self.height[col] >= 3
            && self.board[self.height[col] - 1][col] == self.current
            && self.board[self.height[col] - 2][col] == self.current
            && self.board[self.height[col] - 3][col] == self.current;
    }

    fn check_horizontal(&self, col: usize) -> bool {
        let mut num_stones: u8 = 0;
        let mut x = col + 1;
        while x < WIDTH && self.board[self.height[col]][x] == self.current {
            num_stones += 1;
            x += 1;
        }
        if col >= 1 {
            x = col - 1;
            while self.board[self.height[col]][x] == self.current {
                num_stones += 1;
                if x == 0 {
                    break;
                }
                x -= 1;
            }
        }
        return num_stones >= 3;
    }

    fn check_diagonal(&self, col: usize) -> bool {
        let mut num_stones: u8 = 0;
        let mut x = col + 1;
        let mut y = self.height[col] + 1;
        while x < WIDTH && y < HEIGHT && self.board[y][x] == self.current {
            num_stones += 1;
            x += 1;
            y += 1;
        }
        if col >= 1 && self.height[col] >= 1 {
            x = col - 1;
            y = self.height[col] - 1;
            while self.board[y][x] == self.current {
                num_stones += 1;
                if x == 0 || y == 0 {
                    break;
                }
                x -= 1;
                y -= 1;
            }
        }
        if num_stones >= 3 {
            return true;
        }

        num_stones = 0;
        if self.height[col] >= 1 {
            x = col + 1;
            y = self.height[col] - 1;
            while x < WIDTH && self.board[y][x] == self.current {
                num_stones += 1;
                if y == 0 {
                    break;
                }
                x += 1;
                y -= 1;
            }
        }
        if col >= 1 {
            x = col - 1;
            y = self.height[col] + 1;
            while y < HEIGHT && self.board[y][x] == self.current {
                num_stones += 1;
                if x == 0 {
                    break;
                }
                x -= 1;
                y += 1;
            }
        }
        return num_stones >= 3;
    }
}

fn load_from_file(path: &str) -> (Vec<Position>, Vec<i8>) {
    let content = fs::read_to_string(path).expect("File could not be read.");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut positions: Vec<Position> = Vec::new();
    let mut evaluations: Vec<i8> = Vec::new();
    for line in lines {
        let s: Vec<&str> = line.trim().split(" ").collect();
        positions.push(Position::from_sequence(s[0]));
        evaluations.push(s[1].parse().unwrap());
    }

    return (positions, evaluations);
}

fn main() {
    println!("Select a level of positions [1-6]: ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not read input");
    let level: i8 = input.trim().parse().expect("Could not parse input");
    let (positions, evaluations) = match level {
        1 => load_from_file("test_files/Test_L3_R1"),
        2 => load_from_file("test_files/Test_L2_R1"),
        3 => load_from_file("test_files/Test_L2_R2"),
        4 => load_from_file("test_files/Test_L1_R1"),
        5 => load_from_file("test_files/Test_L1_R2"),
        6 => load_from_file("test_files/Test_L1_R3"),
        _ => panic!("Unknown level!"),
    };
    let mut score = 0;
    let now = Instant::now();
    for (position, eval) in zip(positions, evaluations) {
        let e = position.negamax();
        if e == eval {
            score += 1;
        }
    }
    let elapsed = now.elapsed();
    println!("Correctly evaluated: {}", score);
    println!("Elapsed time: {:.2?}", elapsed);
}
