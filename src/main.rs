use std::env;

mod chip8;
mod display;
mod font;
mod register;
mod rom;

use crate::display::Display;

const TICKS_PER_FRAME: u32 = 10; // Adjust this value to control emulation speed

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <path to ROM>");
        return ;
    }
    let mut display = Display::new();

    let mut chip8 = chip8::Chip8::new();
    let buffer = rom::load(&args[1]).expect("Failed to load ROM file");
    chip8.load(&buffer);

    'gameloop: loop {
        if !display.handle_events(&mut chip8) {
            break 'gameloop;
        }
        for _ in 0..TICKS_PER_FRAME {
            chip8.emulate();
        }
        chip8.emulate_timers();
        display.draw(&chip8);
    }
}
