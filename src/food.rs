use core::fmt::Debug;
use embedded_graphics as eg;
use panic_halt as _;

use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{PrimitiveStyle, Rectangle, Styled};

use embedded_graphics::primitives::StyledDrawable;

// pseudo-random number generation
use oorandom;
use oorandom::Rand32;

const DISPLAY_WIDTH: u32 = 320;
const DISPLAY_HEIGHT: u32 = 240;
const CELL_SIZE: u32 = 10;
const GRID_WIDTH: u32 = DISPLAY_WIDTH / CELL_SIZE as u32;
const GRID_HEIGHT: u32 = DISPLAY_HEIGHT / CELL_SIZE as u32;

pub struct Food<'a> {
    pub sprite: Styled<Rectangle, PrimitiveStyle<Rgb565>>,
    rng: Rand32,
    food_style: &'a PrimitiveStyle<Rgb565>,
    snake_style: &'a PrimitiveStyle<Rgb565>,
}

impl<'a> Food<'a> {
    pub fn init_and_draw<D>(
        seed: u64,
        food_style: &'a PrimitiveStyle<Rgb565>,
        snake_style: &'a PrimitiveStyle<Rgb565>,
        display: &mut D,
    ) -> Self
    where
        D: DrawTarget<Color = Rgb565>,
        <D as DrawTarget>::Error: Debug,
    {
        let mut rng = Rand32::new(seed);
        let x = (rng.rand_range(0..GRID_WIDTH) * CELL_SIZE) as i32;
        let y = (rng.rand_range(0..GRID_HEIGHT) * CELL_SIZE) as i32;
        let sprite = Rectangle::new(Point::new(x, y), Size::new(CELL_SIZE, CELL_SIZE))
            .into_styled(*food_style);
        sprite.draw(display).unwrap();
        Self {
            sprite,
            rng,
            food_style,
            snake_style,
        }
    }

    pub fn respawn<D>(&mut self, display: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
        <D as DrawTarget>::Error: Debug,
    {
        // clear previous food sprite
        self.sprite
            .primitive
            .draw_styled(self.snake_style, display)
            .unwrap();
        let x = (self.rng.rand_range(0..GRID_WIDTH) * CELL_SIZE) as i32;
        let y = (self.rng.rand_range(0..GRID_HEIGHT) * CELL_SIZE) as i32;
        let sprite = Rectangle::new(Point::new(x, y), Size::new(CELL_SIZE, CELL_SIZE))
            .into_styled(*self.food_style);
        self.sprite = sprite;
        self.sprite.draw(display).unwrap();
    }
}
