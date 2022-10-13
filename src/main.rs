#![no_std]
#![no_main]

pub mod food;
pub mod snake;
pub mod status;

use crate::food::*;
use crate::snake::*;
use crate::status::*;

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle};
use eg::text::{Alignment, Text};

use cortex_m::interrupt::{free as disable_interrupts, CriticalSection};
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{interrupt, CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{button_interrupt, Button, ButtonController, ButtonEvent};

// Queues used for button stuff (just normal wio stuff) and then i use it for managing clear queue
use heapless::{consts::U8, spsc::Queue};

const CELL_SIZE: u32 = 10;

static mut BUTTON_CTRLR: Option<ButtonController> = None;
static mut Q: Queue<ButtonEvent, U8> = Queue(heapless::i::Queue::new());

button_interrupt!(
    BUTTON_CTRLR,
    unsafe fn on_button_event(_cs: &CriticalSection, event: ButtonEvent) {
        let mut q = Q.split().0;
        q.enqueue(event).ok();
    }
);

const TITLE_TEXT: &str = "Wio Terminal Snake";
const CLICK_TO_START: &str = "Click to start";
const SNAKE_GAME: &str = "Snake game";

#[entry]
fn main() -> ! {
    // Initial initializations
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let sets = wio::Pins::new(peripherals.PORT).split();
    let mut consumer = unsafe { Q.split().1 };

    // initializing styles
    let black_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();

    let menu_style_marine = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::CSS_AQUAMARINE)
        .build();

    let food_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::CSS_ORANGE)
        .build();

    let snake_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::WHITE)
        .build();

    // Initialize the ILI9341-based LCD display. Create a black backdrop the size of
    // the screen, load an image of Ferris from a RAW file, and draw it to the
    // screen.
    // By default, the display is in the LandscapeFlipped orientation.
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            100.mhz(),
            &mut delay,
        )
        .unwrap();

    // Initializing game status manager
    let mut game_status = Status::new();

    // initializing buttons
    let button_ctrlr = sets
        .buttons
        .init(peripherals.EIC, &mut clocks, &mut peripherals.MCLK);
    let nvic = &mut core.NVIC;
    disable_interrupts(|_| unsafe {
        button_ctrlr.enable(nvic);
        BUTTON_CTRLR = Some(button_ctrlr);
    });

    let mut init_flag: bool = true;

    loop {
        match game_status.get_state() {
            State::Menu => {
                'menu: loop {
                    let character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::BLUE);

                    let yoffset = 14;

                    if init_flag {
                        init_flag = false;
                        // Draw a 3px wide outline around the display.
                        display
                            .bounding_box()
                            .into_styled(snake_style)
                            .draw(&mut display)
                            .unwrap();

                        // Draw a triangle.
                        Triangle::new(
                            Point::new(16, 16 + yoffset),
                            Point::new(16 + 16, 16 + yoffset),
                            Point::new(16 + 8, yoffset),
                        )
                        .into_styled(menu_style_marine)
                        .draw(&mut display)
                        .unwrap();

                        // Draw a filled square
                        Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
                            .into_styled(menu_style_marine)
                            .draw(&mut display)
                            .unwrap();

                        // Draw a circle with a 3px wide stroke.
                        Circle::new(Point::new(88, yoffset), 17)
                            .into_styled(menu_style_marine)
                            .draw(&mut display)
                            .unwrap();

                        // Draw centered text.
                        Text::with_alignment(
                            TITLE_TEXT,
                            display.bounding_box().center() + Point::new(0, 15),
                            character_style,
                            Alignment::Center,
                        )
                        .draw(&mut display)
                        .unwrap();

                        delay.delay_ms(100u8);
                    }
                    if let Some(press) = consumer.dequeue() {
                        // match with button
                        match press.button {
                            Button::Down => {
                                continue 'menu;
                            }
                            Button::Up => {
                                continue 'menu;
                            }
                            Button::Left => {
                                continue 'menu;
                            }
                            Button::Right => {
                                continue 'menu;
                            }
                            _ => {
                                game_status.set_state(State::Snake(true));
                                break 'menu;
                            }
                        }
                    }
                }
            }
            // Click to start game
            State::Snake(is_continued) => {
                if is_continued {
                    // Initializing backdrop and initial sprite render
                    Rectangle::new(Point::new(0, 0), Size::new(360, 240))
                        .into_styled(black_style)
                        .draw(&mut display)
                        .unwrap();

                    let mut player = Snake::init();
                    player.translate(&mut display);

                    let mut food = Food::init_and_draw(5, &food_style, &snake_style, &mut display);

                    let mut flag_incr_snake_len_this_iter = false;
                    let mut delay_gap: u8 = 100;

                    'game: loop {
                        if let Some(press) = consumer.dequeue() {
                            // match with button
                            match press.button {
                                Button::Down => {
                                    player.set_direction(Direction::Down);
                                }
                                Button::Up => {
                                    player.set_direction(Direction::Up);
                                }
                                Button::Left => {
                                    player.set_direction(Direction::Left);
                                }
                                Button::Right => {
                                    player.set_direction(Direction::Right);
                                }
                                _ => {}
                            }
                        }

                        if player.is_player_eat_food(&food) {
                            food.respawn(&mut display);
                            flag_incr_snake_len_this_iter = true;
                            if delay_gap > 40 {
                                delay_gap -= 5;
                            }
                        }

                        if player.is_self_intersecting() {
                            game_status.set_state(State::Snake(false));
                            break 'game;
                            // loop {} // effectively exiting...
                        }

                        // clear previously printed sprite
                        player
                            .cells_queue
                            .enqueue((
                                player.head_sprite.primitive.top_left.x,
                                player.head_sprite.primitive.top_left.y,
                            ))
                            .unwrap();
                        player.translate(&mut display);
                        // if snake eats food, we don't clear the coord in the queue effectively increasing the snake's size
                        if !flag_incr_snake_len_this_iter {
                            Rectangle::new(
                                Point::from(player.cells_queue.dequeue().unwrap()),
                                Size::new(CELL_SIZE, CELL_SIZE),
                            )
                            .into_styled(black_style)
                            .draw(&mut display)
                            .unwrap();
                        }
                        flag_incr_snake_len_this_iter = false;
                        delay.delay_ms(delay_gap);
                    }
                } else {
                    game_status.set_state(State::Menu);
                    init_flag = true;
                    // ToDo: Restart or Menu
                }
            }
        }
    }
}
