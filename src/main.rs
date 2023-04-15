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

}
