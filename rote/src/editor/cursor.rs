pub struct Cursor {
    row: usize,
    col: usize
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { row: 0, col: 0 }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    pub fn set_position(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn move_up(&mut self) {
        if self.row > 0 {
            self.row -= 1;
        }
    }

    pub fn move_down(&mut self) {
        self.row += 1;
    }

    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    pub fn move_right(&mut self) {
        self.col += 1;
    }
}