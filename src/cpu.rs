use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::env;
use random;
use keyboard::Keypad;
use display::Display;

const CHIP8_FONTSET:[u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip {
    op_code: u16,
    memory: [u8; 4096],
    V: [u8; 16],
    index_register: usize,
    program_counter: usize,
    stack: [u16; 16],
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    pub key: Keypad,
    pub display: Display,
}

impl Chip {

    pub fn init() -> Chip {
        let loaded_memory = Chip::load_fontset();
        Chip {
            op_code: 0,
            program_counter: 0x200,
            index_register: 0x200,
            memory: loaded_memory,
            V: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            key: Keypad::new(),
            stack: [0; 16],
            display: Display::new()
        }
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        self.fetch_opcode();
        
        // Decode Opcode
        // Execute Opcode
        self.execute_opcode();
        
        // Update Timers
        if self.delay_timer > 0 { self.delay_timer -= 1; }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 { println!("BEEEEP\n"); }
            self.sound_timer -= 1;
        }

        for i in 0..10000 { }
    }

    pub fn load_game(&mut self, game_name: &'static str) {
        let mut path = env::current_dir().unwrap();
        path.push(game_name.trim());
        let mut reader = File::open(&path).unwrap();
        let mut game_data = Vec::new();
        reader.read_to_end(&mut game_data).expect("Failure to read file");

        self.load_to_memory(&mut game_data);
    }
    fn load_fontset() -> [u8; 4096] {
        let mut loaded_memory = [0u8; 4096];
        for i in 0..80 {
            loaded_memory[i] = CHIP8_FONTSET[i];
        }
        loaded_memory
    }

    fn load_to_memory(&mut self, game_data: &mut Vec<u8>) {

        for (i, byte) in game_data.iter().enumerate() {
            self.memory[self.program_counter] = byte.clone();
            self.program_counter += 1;
        }

        self.program_counter = 0x200;
    }

    fn fetch_opcode(&mut self) {
        self.op_code = (self.memory[self.program_counter] as u16) << 8 | (self.memory[self.program_counter + 1] as u16);
    }

    fn execute_opcode(&mut self) {
        match self.op_code & 0xf000 {
            0x0000 => {
                match self.op_code & 0x000F {
                    0x0000 => {
                        // Clear Display
                        self.display.clear();
                        self.program_counter += 2;
                    },
                    0x000E => {
                        // Return
                        self.stack_pointer -= 1;
                        self.program_counter = self.stack[self.stack_pointer] as usize;
                    },
                    _ => {
                        println!("Not Implemented: {}", self.op_code);
                    }
                }
            },
            0x1000 => {
                // Jump to Address
                self.program_counter = self.op_nnn() as usize;
            },
            0x2000 => {
                // Call Subroutine
                self.stack[self.stack_pointer] = self.program_counter as u16;
                self.stack_pointer += 1;
                self.program_counter = self.op_nnn() as usize;
            },
            0x3000 => {
                // Skips next instruction if VX == NN
                self.program_counter += if self.V[self.op_x()] == self.op_nn() { 4 } else { 2 }
            },
            0x4000 => {
                // Skips next instruction if VX != NN
                self.program_counter += if self.V[self.op_x()] != self.op_nn() { 4 } else { 2 }
            },
            0x5000 => {
                // Skips next instruction if VX equals VY
                self.program_counter += if self.V[self.op_x()] == self.V[self.op_y()] { 4 } else { 2 }
            },
            0x6000 => {
                // Set VX to NN
                self.V[self.op_x()] = self.op_nn();
                self.program_counter += 2;
            },
            0x7000 => {
                // Add NN to VX
                self.V[self.op_x()] += self.op_nn();
                self.program_counter += 2;
            },
            0x8000 => {
                match self.op_code & 0x000F {
                    0 => {
                        // Move
                        self.V[self.op_x()] = self.V[self.op_y()];
                    },
                    1 => {
                        // OR
                        self.V[self.op_x()] |= self.V[self.op_y()];
                    },
                    2 => {
                        // AND
                        self.V[self.op_x()] &= self.V[self.op_y()];
                    },
                    3 => {
                        // Xor
                        self.V[self.op_x()] ^= self.V[self.op_y()];
                    },
                    4 => {
                        // Add
                        self.V[self.op_x()] += self.V[self.op_y()];
                        self.V[15] = if self.V[self.op_x()] < self.V[self.op_y()] { 1 } else { 0 };
                    },
                    5 => {
                        // Sub
                        self.V[15] = if self.V[self.op_y()] > self.V[self.op_x()] { 0 } else { 1 };
                        self.V[self.op_x()] -= self.V[self.op_y()];
                    },
                    6 => {
                        // Shift Right
                        self.V[15] = self.V[self.op_x()] & 0x1;
                        self.V[self.op_x()] >>= 1;
                    },
                    7 => {
                        //Reverse Sub
                        self.V[15] = if self.V[self.op_x()] > self.V[self.op_y()] { 0 } else { 1 };
                        self.V[self.op_x()] = self.V[self.op_y()] - self.V[self.op_x()];
                    },
                    0xE => {
                        // Shift Left
                        self.V[15] = self.V[self.op_x()] >> 7;
                        self.V[self.op_x()] <<= 1;
                    },
                    _ => {
                        println!("Unrecognized op_code: {}", self.op_code);
                    }
                }

                 self.program_counter += 2;
            },
            0x9000 => {
                // Skip if VX != VY
                self.program_counter += if self.V[self.op_x()] != self.V[self.op_y()] { 4 } else { 2 }
            },
            0xA000 => {
                //Load index_counter
                self.index_register = self.op_n() as usize;
                self.program_counter += 2;
            },
            0xB000 => {
                //Jump + Zero
                self.program_counter = (self.op_nnn() + (self.V[0] as u16)) as usize;
            },
            0xC000 => {
                // Random
                self.V[self.op_x()] = self.op_nn() & random::<u8>();
                self.program_counter += 2;
            },
            0xD000 => {
                // Draw
                let from = self.index_register;
                let to = from + (self.op_n() as usize);
                let x = self.V[self.op_x()];
                let y = self.V[self.op_y()];

                self.V[15] = self.display.draw(x as usize, y as usize, &self.memory[from..to]);
                self.program_counter += 2;
            },
            0xE000 => {
                let V = self.V[self.op_x()] as usize;
                self.program_counter += match self.op_code & 0x00FF {
                    0x9E => {
                        // Skip if Pressed
                        if self.key.key_pressed(V) { 4 } else { 2 }
                    },
                    0xA1 => {
                        // Skip if not Pressed
                        if !self.key.key_pressed(V) { 4 } else { 2 }
                    },
                    _ => {
                        println!("op_code not recognized: {}", self.op_code);
                        0
                    }
                }
            },
            0xF000 => {
                match self.op_code & 0x00FF {
                    0x07 => {
                        // Load Delay Timer
                        self.V[self.op_x()] = self.delay_timer;
                    },
                    0x0A => {
                        // Wait for Keypress
                    },
                    0x15 => {
                        // Set delay_timer
                        self.delay_timer = self.V[self.op_x()];
                    },
                    0x18 => {
                        // Set sound_timer
                        self.sound_timer = self.V[self.op_x()];
                    },
                    0x1E => {
                        // Add to index_register
                        self.index_register += self.V[self.op_x()] as usize;
                    },
                    0x29 => {
                        // Load sprite
                        self.index_register = (self.V[self.op_x()] as usize) * 5;
                    },
                    0x33 => {
                        // BCD Representation
                        self.memory[self.index_register] = self.V[self.op_x()] / 100;
                        self.memory[self.index_register + 1] = (self.V[self.op_x()] / 10) % 10;
                        self.memory[self.index_register + 2] = (self.V[self.op_x()] % 100) % 10;
                    },
                    0x55 => {
                        // Store Registers
                        for i in 0..self.op_x() + 1 {
                            self.memory[self.index_register + i] = self.V[i]
                        }
                        self.index_register += self.op_x() + 1;
                    },
                    0x65 => {
                        // Load Registers
                        for i in 0..self.op_x() + 1 {
                            self.V[i] = self.memory[self.index_register + i]
                        }

                        self.index_register += self.op_x() + 1;
                    },
                    _ => {
                        println!("Unknown op_code: {}", self.op_code);
                    }
                }
                self.program_counter += 2;
            },
            _ => {
                println!("Op_code doesn't exist: {}", self.op_code);
            }
        }
    }

    fn op_x(&self) -> usize {
        ((self.op_code & 0x0F00) >> 8) as usize
    }
    fn op_y(&self) -> usize {
        ((self.op_code & 0x00F0) >> 4) as usize
    }
    fn op_n(&self) -> u8 {
        (self.op_code & 0x000F) as u8
    }
    fn op_nn(&self) -> u8 {
        (self.op_code & 0x00FF) as u8
    }
    fn op_nnn(&self) -> u16 {
        self.op_code & 0x0FFF
    }

}