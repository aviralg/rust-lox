#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
    pub pos: usize,
}

impl Position {
    pub fn new(row: usize, col: usize, pos: usize) -> Position {
        Position { row, col, pos }
    }

    pub fn new2(elts: (usize, usize, usize)) -> Position {
        Position {
            row: elts.0,
            col: elts.1,
            pos: elts.2,
        }
    }

    pub fn add_col(&self, col: usize) -> Position {
        Position {
            row: self.row,
            col: self.col + col,
            pos: self.pos + col,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Location {
    pub beg: Position,
    pub end: Position,
}

impl Location {
    pub fn new(beg: Position, end: Position) -> Self {
        Location { beg, end }
    }

    pub fn new2(beg: (usize, usize, usize), end: (usize, usize, usize)) -> Self {
        Location {
            beg: Position::new2(beg),
            end: Position::new2(end),
        }
    }

    pub fn zero(row: usize, col: usize, pos: usize) -> Self {
        let pos1 = Position { row, col, pos };
        let pos2 = pos1.clone();
        Self::new(pos1, pos2)
    }
}
