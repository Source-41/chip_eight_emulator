extern crate rand;
extern crate sdl2;

use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::env;
use rand::*;
use sdl2::event::Event;

mod display;
mod keyboard;
mod cpu;

use cpu::Chip;

fn main() {
    // setupGraphics();
    // setupInput();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut main_chip: Chip = Chip::init(&sdl_context);

    main_chip.load_game("pong");

    sdl2::init();
    // main_chip.emulate_cycle();
}
