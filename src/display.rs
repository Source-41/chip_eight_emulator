pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

#[derive(Clone)]
pub struct Display {
    pub gfx: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub dirty: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            gfx: [[0u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            dirty: true
        }
    }

    pub fn clear(&mut self) {
        self.gfx = [[0u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
        self.dirty = true;
    }

    pub fn draw(&mut self, xpos: usize, ypos: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0u8;
        let h = sprite.len();

        for j in 0..h {
            for i in 0..8 {
                let y = (ypos + j) % DISPLAY_HEIGHT;
                let x = (xpos + i) % DISPLAY_WIDTH;

                if (sprite[j] & (0x80 >> i)) != 0x00 {
                    if self.gfx[y][x] == 0x01 { collision = 1; }
                    self.gfx[y][x] ^= 0x01;
                }
            }
        }
        self.dirty = true;

        collision
    }

    pub fn draw_screen ()
}