const CHIP8_FONTSET:[u16; 80] = [
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

fn main() {
    // setupGraphics();
    // setupInput();

    let main_chip: Chip = Chip::init();

    main_chip.load_game("pong");
}

struct Chip {
    op_code: u16,
    program_counter: u16,
    index_register: u16,
    memory: [u16; 4096],
    V: [u16; 16],
    gfx: [u16; 64 * 32],
    delay_timer: u16,
    sound_timer: u16,
    stack: [u16; 16],
    stack_pointer: u16,
    key: [u16; 16],
}

impl Chip {

    fn init() -> Chip {
        let loaded_memory = Chip::load_fontset();
        Chip {
            op_code: 0,
            program_counter: 0x200,
            index_register: 0,
            memory: loaded_memory,
            V: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            key: [0; 16],
            gfx: [0; 64 *32],
            stack: [0; 16]
        }
    }

    fn load_fontset() -> [u16; 4096] {
        let mut loaded_memory = [0u16; 4096];
        for i in 0..80 {
            loaded_memory[i] = CHIP8_FONTSET[i];
        }
        loaded_memory
    }
    fn emulate_cycle(&mut self) {
        // Fetch Opcode
        self.op_code = self.memory[self.program_counter as usize] << 8 | self.memory[(self.program_counter + 1u16) as usize];
        // Decode Opcode
        match self.op_code {
            0xA000 => {
                self.index_register = self.op_code & 0x0FFF;
            },
            _ => println!("Unknown op_code: 0x{}", self.op_code),
        }
        // Execute Opcode

        // Update Timers
    }

    fn load_game(&self, game_name: &'static str) {

    }
}
