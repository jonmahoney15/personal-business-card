use esp_hal::rng::Rng;

use crate::snake::{Direction, Snake};

pub struct Game {
    pub score: u8,
    snake: Snake,
    food_x: u8,
    food_y: u8,
    running: bool,
    rng: Rng,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            snake: Snake::new(0, 0, Direction::Right),
            food_x: 0,
            food_y: 0,
            running: true,
            score: 0,
            rng: Rng::new(),
        };
        game.place_food();
        game
    }

    fn place_food(&mut self) {
        for _ in 0..64 {
            let x = (self.rng.random() % 8) as u8;
            let y = (self.rng.random() % 8) as u8;
            if !self.snake.occupies(x, y) {
                self.food_x = x;
                self.food_y = y;
                return;
            }
        }

        self.running = false;
    }

    pub fn change_direction(&mut self, dir: Direction) {
        self.snake.set_direction(dir);
    }

    pub fn tick(&mut self) {
        if !self.is_running() {
            return;
        }

        self.snake.advance();

        if self.snake.is_outside() || self.snake.collides_with_self() {
            self.running = false;
            return;
        }

        if self.snake.head_x == self.food_x && self.snake.head_y == self.food_y {
            self.score += 1;
            self.snake.grow();
            self.place_food();
        }
    }

    pub fn frame(&self) -> [u8; 8] {
        if !self.is_running() {
            return [0u8; 8];
        }

        let mut frame = [0u8; 8];

        frame[self.snake.head_y as usize] |= 1 << self.snake.head_x;

        for &(x, y) in self.snake.body_segments() {
            frame[y as usize] |= 1 << x;
        }

        frame[self.food_y as usize] |= 1 << self.food_x;

        rotate_for_display(&frame)
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn reset(&mut self) {
        self.snake = Snake::new(0, 0, Direction::Right);
        self.running = true;
        self.score = 0;
        self.place_food();
    }
}

fn rotate_for_display(frame: &[u8; 8]) -> [u8; 8] {
    let mut rotated = [0u8; 8];
    for y in 0..8usize {
        for x in 0..8u8 {
            if frame[y] & (1 << x) != 0 {
                rotated[7 - x as usize] |= 1 << (7 - y);
            }
        }
    }
    rotated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_for_display_of_empty_frame_is_empty() {
        let frame = [0u8; 8];
        assert_eq!(rotate_for_display(&frame), [0u8; 8]);
    }

    #[test]
    fn rotate_for_display_of_fully_lit_frame_stays_fully_lit() {
        let frame = [0xFFu8; 8];
        assert_eq!(rotate_for_display(&frame), [0xFFu8; 8]);
    }

    #[test]
    fn rotate_for_display_maps_top_left_pixel_to_bottom_left() {
        // Pixel at row 0 (y=0), column 0 (x=0, LSB).
        let mut frame = [0u8; 8];
        frame[0] = 0b0000_0001;

        let rotated = rotate_for_display(&frame);

        let mut expected = [0u8; 8];
        expected[7] = 0b1000_0000;
        assert_eq!(rotated, expected);
    }

    #[test]
    fn rotate_for_display_maps_top_right_pixel_to_top_left() {
        // Pixel at row 0 (y=0), column 7 (x=7, MSB).
        let mut frame = [0u8; 8];
        frame[0] = 0b1000_0000;

        let rotated = rotate_for_display(&frame);

        let mut expected = [0u8; 8];
        expected[0] = 0b1000_0000;
        assert_eq!(rotated, expected);
    }

    #[test]
    fn rotate_for_display_maps_bottom_right_pixel_to_top_right() {
        // Pixel at row 7 (y=7), column 7 (x=7).
        let mut frame = [0u8; 8];
        frame[7] = 0b1000_0000;

        let rotated = rotate_for_display(&frame);

        let mut expected = [0u8; 8];
        expected[0] = 0b0000_0001;
        assert_eq!(rotated, expected);
    }

    #[test]
    fn rotate_for_display_maps_bottom_left_pixel_to_bottom_right() {
        // Pixel at row 7 (y=7), column 0 (x=0).
        let mut frame = [0u8; 8];
        frame[7] = 0b0000_0001;

        let rotated = rotate_for_display(&frame);

        let mut expected = [0u8; 8];
        expected[7] = 0b0000_0001;
        assert_eq!(rotated, expected);
    }

    #[test]
    fn rotate_for_display_maps_interior_pixel_correctly() {
        // Pixel at row 3 (y=3), column 5 (x=5).
        let mut frame = [0u8; 8];
        frame[3] = 1 << 5;

        let rotated = rotate_for_display(&frame);

        // rotated[7 - x] |= 1 << (7 - y) => rotated[2] |= 1 << 4
        let mut expected = [0u8; 8];
        expected[2] = 1 << 4;
        assert_eq!(rotated, expected);
    }

    #[test]
    fn rotate_for_display_preserves_pixel_count_for_sparse_frame() {
        let mut frame = [0u8; 8];
        frame[1] = 0b0010_0100; // two pixels in row 1
        frame[6] = 0b0000_0001; // one pixel in row 6

        let rotated = rotate_for_display(&frame);

        let lit_count: u32 = rotated.iter().map(|row| row.count_ones()).sum();
        assert_eq!(lit_count, 3);
    }
}
