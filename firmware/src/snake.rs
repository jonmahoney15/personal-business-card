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
            Direction::Left => self.head_x = self.head_x.wrapping_sub(1),
            Direction::Right => self.head_x = self.head_x.wrapping_add(1),
            Direction::Up => self.head_y = self.head_y.wrapping_sub(1),
            Direction::Down => self.head_y = self.head_y.wrapping_add(1),
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

fn opposite_direction(dir: Direction) -> Direction {
    return match dir {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_initializes_head_direction_and_empty_body() {
        let snake = Snake::new(3, 5, Direction::Up);

        assert_eq!(snake.head_x, 3);
        assert_eq!(snake.head_y, 5);
        assert_eq!(snake.body_segments(), &[]);
    }

    #[test]
    fn set_direction_updates_to_a_perpendicular_direction() {
        let mut snake = Snake::new(0, 0, Direction::Right);

        snake.set_direction(Direction::Up);
        snake.advance();

        assert_eq!(snake.head_x, 0);
        assert_eq!(snake.head_y, u8::MAX);
    }

    #[test]
    fn set_direction_ignores_direct_reversal() {
        let mut snake = Snake::new(4, 4, Direction::Right);

        // Attempting to reverse straight into itself must be ignored.
        snake.set_direction(Direction::Left);
        snake.advance();

        // Direction should still be Right, so the head moves right, not left.
        assert_eq!(snake.head_x, 5);
        assert_eq!(snake.head_y, 4);
    }

    #[test]
    fn set_direction_allows_reversal_after_a_turn() {
        let mut snake = Snake::new(4, 4, Direction::Right);

        snake.set_direction(Direction::Up);
        snake.set_direction(Direction::Left);
        snake.advance();

        assert_eq!(snake.head_x, 3);
        assert_eq!(snake.head_y, 4);
    }

    #[test]
    fn advance_moves_head_in_each_direction() {
        let mut right = Snake::new(3, 3, Direction::Right);
        right.advance();
        assert_eq!((right.head_x, right.head_y), (4, 3));

        let mut left = Snake::new(3, 3, Direction::Left);
        left.advance();
        assert_eq!((left.head_x, left.head_y), (2, 3));

        let mut up = Snake::new(3, 3, Direction::Up);
        up.advance();
        assert_eq!((up.head_x, up.head_y), (3, 2));

        let mut down = Snake::new(3, 3, Direction::Down);
        down.advance();
        assert_eq!((down.head_x, down.head_y), (3, 4));
    }

    #[test]
    fn advance_without_body_does_not_grow_trail() {
        let mut snake = Snake::new(0, 0, Direction::Right);

        snake.advance();
        snake.advance();

        assert_eq!(snake.body_segments(), &[]);
    }

    #[test]
    fn advance_shifts_body_to_follow_the_head() {
        let mut snake = Snake::new(0, 0, Direction::Right);

        snake.advance(); // head -> (1, 0)
        snake.grow(); // body: [(1, 0)]
        snake.advance(); // head -> (2, 0), body: [(1, 0)]

        assert_eq!(snake.head_x, 2);
        assert_eq!(snake.head_y, 0);
        assert_eq!(snake.body_segments(), &[(1, 0)]);

        snake.grow(); // body: [(1, 0), (1, 0)]
        snake.advance(); // head -> (3, 0), body shifts: [(2, 0), (1, 0)]

        assert_eq!(snake.head_x, 3);
        assert_eq!(snake.head_y, 0);
        assert_eq!(snake.body_segments(), &[(2, 0), (1, 0)]);
    }

    #[test]
    fn is_outside_detects_wraparound_past_the_left_and_top_edges() {
        let mut left_edge = Snake::new(0, 0, Direction::Left);
        left_edge.advance();
        assert!(left_edge.is_outside());

        let mut top_edge = Snake::new(0, 0, Direction::Up);
        top_edge.advance();
        assert!(top_edge.is_outside());
    }

    #[test]
    fn is_outside_detects_overflow_past_the_right_and_bottom_edges() {
        let mut right_edge = Snake::new(7, 0, Direction::Right);
        right_edge.advance();
        assert!(right_edge.is_outside());

        let mut bottom_edge = Snake::new(0, 7, Direction::Down);
        bottom_edge.advance();
        assert!(bottom_edge.is_outside());
    }

    #[test]
    fn is_outside_is_false_within_bounds() {
        let snake = Snake::new(7, 7, Direction::Right);
        assert!(!snake.is_outside());

        let origin = Snake::new(0, 0, Direction::Right);
        assert!(!origin.is_outside());
    }

    #[test]
    fn grow_from_empty_body_duplicates_current_head_position() {
        let mut snake = Snake::new(2, 2, Direction::Right);

        snake.grow();

        assert_eq!(snake.body_segments(), &[(2, 2)]);
    }

    #[test]
    fn grow_from_non_empty_body_duplicates_the_tail_segment() {
        let mut snake = Snake::new(0, 0, Direction::Right);
        snake.advance(); // head -> (1, 0)
        snake.grow(); // body: [(1, 0)]
        snake.advance(); // head -> (2, 0), body: [(1, 0)]
        snake.grow(); // body: [(1, 0), (1, 0)] duplicates current tail

        assert_eq!(snake.body_segments(), &[(1, 0), (1, 0)]);
    }

    #[test]
    #[should_panic]
    fn grow_beyond_body_capacity_panics() {
        let mut snake = Snake::new(0, 0, Direction::Right);

        // The body buffer only has capacity for 63 segments (a full 8x8
        // board minus the head), so the 64th grow() call is expected to
        // index out of bounds. This documents existing behavior/limits.
        for _ in 0..64 {
            snake.grow();
        }
    }

    #[test]
    fn occupies_matches_head_position() {
        let snake = Snake::new(5, 6, Direction::Right);

        assert!(snake.occupies(5, 6));
        assert!(!snake.occupies(0, 0));
    }

    #[test]
    fn occupies_matches_body_positions() {
        let mut snake = Snake::new(0, 0, Direction::Right);
        snake.advance(); // head -> (1, 0)
        snake.grow(); // body: [(1, 0)]

        assert!(snake.occupies(1, 0));
        assert!(!snake.occupies(2, 2));
    }

    #[test]
    fn collides_with_self_is_false_without_overlap() {
        let mut snake = Snake::new(0, 0, Direction::Right);
        snake.advance();
        snake.grow();

        assert!(!snake.collides_with_self());
    }

    #[test]
    fn collides_with_self_detects_overlap_after_looping_back() {
        let mut snake = Snake::new(0, 0, Direction::Right);

        snake.advance(); // head -> (1, 0)
        snake.grow(); // body: [(1, 0)]

        snake.set_direction(Direction::Down);
        snake.advance(); // head -> (1, 1)
        snake.grow(); // body: [(1, 0), (1, 0)]

        snake.set_direction(Direction::Left);
        snake.advance(); // head -> (0, 1)
        snake.grow(); // body: [(1, 1), (1, 0), (1, 0)]

        snake.set_direction(Direction::Up);
        snake.advance(); // head -> (0, 0)
        assert!(!snake.collides_with_self());
        snake.grow(); // body: [(0, 1), (1, 1), (1, 0), (1, 0)]

        snake.set_direction(Direction::Right);
        snake.advance(); // head -> (1, 0), which is now part of the body

        assert_eq!(snake.head_x, 1);
        assert_eq!(snake.head_y, 0);
        assert!(snake.collides_with_self());
    }
}
