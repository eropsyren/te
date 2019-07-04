pub struct TextView {
    width: usize,
    height: usize,
    cursor_pos: CursorPos,
}

impl TextView {}

struct CursorPos {
    x: usize,
    y: usize,
}

impl CursorPos {
    pub fn new() -> Self {
        CursorPos { x: 0, y: 0 }
    }

    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn right(&mut self, bound: usize) {
        if self.x < bound {
            self.x += 1;
        }
    }

    pub fn up(&mut self, bound: usize) {
        if self.y < bound {
            self.y += 1;
        }
    }

    pub fn down(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
}
