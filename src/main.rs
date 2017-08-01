extern crate rand;
extern crate sdl2;

use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::env;
use rand::*;
use sdl2::event::{Event};

mod display;
mod keyboard;
mod cpu;

use cpu::Chip;

static scale: u32 = 20;

fn main() {
    // setupGraphics();
    // setupInput();
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut timer = ctx.timer().unwrap();

    // Create a window

    let mut window = match video_ctx.window("eg01", 64*scale, 32*scale).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Display the window for 3 seconds

    window.show();
    // timer.delay(3000);

    let mut main_chip: Chip = Chip::init();
    main_chip.load_game("pong");

    let mut events = ctx.event_pump().unwrap();

    'main : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => {
                    break 'main
                },
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    println!("Keycode: {}", keycode);
                    main_chip.key.key_press(keycode, true)
                },
                _ => {

                }
            }
        }
        // main_chip.emulate_cycle();
    }
}
