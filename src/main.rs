const WIDTH: usize = 7;
const HEIGHT: usize = 6;

struct Position {
    board: [[i8; WIDTH]; HEIGHT],
    height: [usize; WIDTH],
    num_moves: u8,
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
            while y < WIDTH && self.board[y][x] == self.current {
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
}
