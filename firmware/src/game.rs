use esp_hal::rng::Rng;

use crate::{
    display::Display,
    snake::{Direction, Snake},
};

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

        Display::turn_on_led(&mut frame, self.snake.head_x, self.snake.head_y);

        for &(x, y) in self.snake.body_segments() {
            Display::turn_on_led(&mut frame, x, y);
        }

        Display::turn_on_led(&mut frame, self.food_x, self.food_y);

        frame
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
