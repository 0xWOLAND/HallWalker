pub struct PawnPlace {
    x: u8,
    y: u8
}

pub struct Pawn {
    index: u8,
    whiteSide: bool,
    isHuman: bool,
    position: PawnPlace,
    endRow: u8,
    wallCount: u8
}

impl PawnPlace {
    pub fn new(_x: u8, _y: u8) -> Self {
        Self {
            x: _x,
            y: _y
        }
    }

    pub fn play(&mut self, dx: u8, dy: u8) -> Self {
        let _x = self.x + dx;
        let _y = self.y + dy;
        let board_width = 0..9;
        assert(board_width.contains(&self.x) && board_width.contains(&self.y), "Illegal move");
        
        Self {
            x: _x,
            y: _y
        }
    }

    pub fn equals(&mut self, x: u8, y: u8){
        return self.x == x && self.y == y;
    }
}

impl Pawn {
    pub fn new(
        index: u8,
        whiteSide: bool,
        isHuman: bool
    ) -> Self {
        // On white side
        if whiteSide {
            return Self {
                index: index,
                whiteSide: whiteSide,
                isHuman: isHuman,
                position: PawnPlace::new(0, 4),
                endRow: 8,
                wallCount: 10
            }
        }
        else {
            return Self {
                index: index,
                whiteSide: whiteSide,
                isHuman: isHuman,
                position: PawnPlace::new(8, 4),
                endRow: 0,
                wallCount: 10
            }
        }
    }
}