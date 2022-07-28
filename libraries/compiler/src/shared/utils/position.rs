#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    pub start: usize,
    pub length: usize,
}

impl Position {
    pub fn empty() -> Position {
        Position {
            start: 0,
            length: 0,
        }
    }
    
    pub fn new(start: usize, length: usize) -> Position {
        Position { start, length, }
    }
    
}