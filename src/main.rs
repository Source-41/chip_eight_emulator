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
    memory: [u8; 4096],
    V: [u8; 16],
    gfx: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    stack_pointer: u16,
    key: [u8; 16],
}

impl Chip {
    fn init() -> Chip {
        
        Chip {
            op_code: 0,
            program_counter: 0x200,
            index_register: 0,
            memory: [0; 4096],
            V: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            key: [0; 16],
            gfx: [0; 64 *32],
            stack: [0; 16]
        }
    }

    fn emulate_cycle() {
        // Fetch Opcode
        // Decode Opcode
        // Execute Opcode

        // Update Timers
    }

    fn load_game(&self, game_name: &'static str) {

    }
}
