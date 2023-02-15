pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const TOTAL_PIXELS: usize = WIDTH * HEIGHT; // 2048

use super::Interpreter;

impl Interpreter
{
    pub fn render(&mut self)
    {
        if self.is_debug() || !self.get_draw_flag()
        {
            return;
        }

        for pixel in 0..2048
        {
            if pixel % WIDTH == 0
            {
                println!();
            }
            if self.get_graphics_at_address(pixel) == 0
            {
                print!(" ");
            }
            else
            {
                print!("▀");
            }
        }
    }
}
