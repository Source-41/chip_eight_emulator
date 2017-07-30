use sdl2::keyboard::Keycode;

pub struct Keypad {
    keys: [bool; 16]
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {keys: [false; 16] }
    }

    pub fn key_pressed(&mut self, index: usize) -> bool {
        self.keys[index]
    }

    pub fn key_press(&mut self, key: Keycode , state: bool) {
        match key {
            Num1 => {

            },
            Num2 => {
                
            }
            _ => {
                println!("Unknown Key: {}", key);
            }
        }
    }
}