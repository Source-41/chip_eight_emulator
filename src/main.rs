extern crate rand;
extern crate sdl2;

use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::env;
use rand::*;
use sdl2::event::{Event,WindowEvent};
use sdl2::surface::{Surface};

mod display;
mod keyboard;
mod cpu;

use cpu::Chip;

static scale: u32 = 20;

fn main() {

    let ctx = sdl2::init().unwrap();
    let mut timer = ctx.timer().unwrap();

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
        main_chip.emulate_cycle();
    }
}