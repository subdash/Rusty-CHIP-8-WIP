use cursive::theme::{Color, ColorStyle};
use cursive::Printer;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::Vec2;
use cursive::XY;

use super::Interpreter;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
// pub const TOTAL_SIZE: usize = WIDTH * HEIGHT;

impl cursive::view::View for Interpreter
{
    fn draw(&self, p: &Printer)
    {
        if !self.draw_flag
        {
            return;
        }

        let white: Color = Color::Rgb(0, 0, 0);
        let black: Color = Color::Rgb(255, 255, 255);
        let white_style: ColorStyle = ColorStyle::new(white, white);
        let black_style: ColorStyle = ColorStyle::new(black, black);    

        for row in 0..HEIGHT
        {
            for col in 0..WIDTH
            {
                let cell = self.pixels[row][col];
                let style = if cell == 0 { white_style } else { black_style };

                // println!("({}, {}) - px {}: {}", x, y, (y * HEIGHT) + x, cell);

                p.with_color(style, |printer| {
                    printer.print((col, row), " ");
                });
            }
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult
    {
        match event
        {
            Event::Refresh =>
            {
                self.fetch();
                self.decode_and_execute();
                self.dec_delay_timer();
                self.dec_sound_timer();
                // self.draw_flag =false;
            
                EventResult::Consumed(None)
            }
            _ => EventResult::Ignored
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 { XY { x: WIDTH, y: HEIGHT } }
}
