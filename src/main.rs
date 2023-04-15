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

}
