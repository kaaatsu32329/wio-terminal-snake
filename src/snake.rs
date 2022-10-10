use crate::food::Food;

use core::fmt::Debug;
use embedded_graphics as eg;
use panic_halt as _;

use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Styled};

// Queues used for button stuff (just normal wio stuff) and then i use it for managing clear queue
use heapless::{consts::U64, spsc::Queue};

const DISPLAY_WIDTH: u32 = 320;
const DISPLAY_HEIGHT: u32 = 240;
const CELL_SIZE: u32 = 10;
const GRID_WIDTH: u32 = DISPLAY_WIDTH / CELL_SIZE as u32;
const GRID_HEIGHT: u32 = DISPLAY_HEIGHT / CELL_SIZE as u32;

pub struct Snake {
    pub head_sprite: Styled<Rectangle, PrimitiveStyle<Rgb565>>,
    snake_direction: Direction,
    pub cells_queue: Queue<(i32, i32), U64>,
}

impl Snake {
    pub fn init() -> Self {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::WHITE)
            .build();
        let position = Point::new(
            ((GRID_WIDTH / 2 - 1) * CELL_SIZE) as i32,
            ((GRID_HEIGHT / 2 - 1) * CELL_SIZE) as i32,
        );
        let sprite = Rectangle::new(position, Size::new(CELL_SIZE, CELL_SIZE)).into_styled(style);
        let cells_queue: Queue<(i32, i32), U64> = Queue::new();
        Self {
            head_sprite: sprite,
            snake_direction: Direction::Down,
            cells_queue,
        }
    }

    pub fn translate<D>(&mut self, display: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
        <D as DrawTarget>::Error: Debug,
    {
        match self.snake_direction {
            Direction::Up => self.head_sprite.primitive.top_left.y -= CELL_SIZE as i32,
            Direction::Down => self.head_sprite.primitive.top_left.y += CELL_SIZE as i32,
            Direction::Left => self.head_sprite.primitive.top_left.x -= CELL_SIZE as i32,
            Direction::Right => self.head_sprite.primitive.top_left.x += CELL_SIZE as i32,
        }

        // code for wrap-around
        if self.head_sprite.primitive.top_left.y < 0 {
            self.head_sprite.primitive.top_left.y = DISPLAY_HEIGHT as i32 - CELL_SIZE as i32;
        }

        if self.head_sprite.primitive.top_left.x < 0 {
            self.head_sprite.primitive.top_left.x = DISPLAY_WIDTH as i32 - CELL_SIZE as i32;
        }

        if self.head_sprite.primitive.top_left.x >= DISPLAY_WIDTH as i32 {
            self.head_sprite.primitive.top_left.x = 0;
        }

        if self.head_sprite.primitive.top_left.y >= DISPLAY_HEIGHT as i32 {
            self.head_sprite.primitive.top_left.y = 0;
        }
        self.head_sprite.draw(display).unwrap();
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.snake_direction == Direction::Up && direction == Direction::Down {
            return;
        }
        if self.snake_direction == Direction::Down && direction == Direction::Up {
            return;
        }
        if self.snake_direction == Direction::Left && direction == Direction::Right {
            return;
        }
        if self.snake_direction == Direction::Right && direction == Direction::Left {
            return;
        }
        self.snake_direction = direction;
    }

    pub fn is_player_eat_food(&self, food: &Food) -> bool {
        !self
            .head_sprite
            .primitive
            .intersection(&food.sprite.primitive)
            .is_zero_sized()
    }

    pub fn is_self_intersecting(&self) -> bool {
        let mut result = false;
        for coord in self.cells_queue.iter() {
            result = result
                || !Rectangle::new(Point::from(*coord), Size::from((CELL_SIZE, CELL_SIZE)))
                    .intersection(&self.head_sprite.primitive)
                    .is_zero_sized()
        }
        result
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
