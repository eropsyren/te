pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { x: 0, y: 0 }
    }

    pub fn origin(&mut self) -> &Self {
        self.x = 0;
        self.y = 0;

        self
    }

    pub fn left(&mut self) -> &Self {
        if self.x > 0 {
            self.x -= 1;
        }

        self
    }

    pub fn right(&mut self) -> &Self {
        self.x += 1;

        self
    }

    pub fn up(&mut self) -> &Self {
        if self.y > 0 {
            self.y -= 1;
        }

        self
    }

    pub fn down(&mut self) -> &Self {
        self.y += 1;

        self
    }
}
