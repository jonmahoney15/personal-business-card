#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Snake {
    pub head_x: u8,
    pub head_y: u8,
    direction: Direction,
    body: [(u8, u8); 63],
    body_len: usize,
}

impl Snake {
    pub fn new(x: u8, y: u8, direction: Direction) -> Self {
        Self {
            head_x: x,
            head_y: y,
            direction,
            body: [(0, 0); 63],
            body_len: 0,
        }
    }

    pub fn set_direction(&mut self, dir: Direction) {
        if self.direction != opposite_direction(dir) {
            self.direction = dir;
        }
    }

    pub fn advance(&mut self) {
        if self.body_len > 0 {
            self.body.copy_within(..self.body_len - 1, 1);
            self.body[0] = (self.head_x, self.head_y);
        }

        match self.direction {
            Direction::Left => self.head_x -= 1,
            Direction::Right => self.head_x += 1,
            Direction::Up => self.head_y += 1,
            Direction::Down => self.head_y -= 1,
        }
    }

    pub fn collides_with_self(&self) -> bool {
        self.body[..self.body_len].contains(&(self.head_x, self.head_y))
    }

    pub fn is_outside(&self) -> bool {
        self.head_x > 7 || self.head_y > 7
    }

    pub fn grow(&mut self) {
        let eaten_point = if self.body_len > 0 {
            self.body[self.body_len - 1]
        } else {
            (self.head_x, self.head_y)
        };

        self.body[self.body_len] = eaten_point;
        self.body_len += 1;
    }

    pub fn body_segments(&self) -> &[(u8, u8)] {
        &self.body[..self.body_len]
    }

    pub fn occupies(&self, x: u8, y: u8) -> bool {
        (self.head_x, self.head_y) == (x, y) || self.body[..self.body_len].contains(&(x, y))
    }
}

fn opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    }
}
