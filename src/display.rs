// use sdl2::video;
// use sdl2::pixels::PixelFormatEnum;
// use sdl2::surface::Surface;

// pub struct Display<'a> {
//     gfx: [[u8; 64]; 32],
//     draw_flag: bool,
//     screen: Surface<'a>
// }

// static scale: isize = 20;

// impl Display<'a> {
//     pub fn new() -> Display<'a> {
//         Display {
//             gfx: [[0; 64]; 32],
//             draw_flag: true,
//             screen: Surface::new(64*scale, 32*scale, PixelFormatEnum::RGB24).unwrap()
//         }
//     }
// }