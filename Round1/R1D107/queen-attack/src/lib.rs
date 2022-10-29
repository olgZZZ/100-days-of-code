pub struct ChessPosition {
    column: i16,
    row: i16
}

pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(column: i16, row: i16) -> Option<Self> {
        if (column < 0) || (row < 0) || (column > 7) || (row > 7) {
            return None;
        }
        Some(ChessPosition { column, row })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        return (self.position.column == other.position.column) ||
            (self.position.row == other.position.row) ||
            (self.position.row - other.position.row).abs() == 
            (self.position.column - other.position.column).abs()
    }
}