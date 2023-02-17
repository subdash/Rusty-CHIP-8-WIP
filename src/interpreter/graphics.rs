use cursive::theme::{Color, ColorStyle};
use cursive::Printer;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::Vec2;
use cursive::XY;

use super::Interpreter;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const TOTAL_SIZE: usize = WIDTH * HEIGHT;

impl Interpreter
{
    pub fn get_px(&self, index: usize) -> u8
    {
        self.pixels[index]
    }

    pub fn set_px(&mut self, index: usize, value: u8)
    {
        self.pixels[index] = value;
    }
}

impl cursive::view::View for Interpreter
{
    fn draw(&self, p: &Printer)
    {
        let white: Color = Color::Rgb(0, 0, 0);
        let black: Color = Color::Rgb(255, 255, 255);
        let white_style: ColorStyle = ColorStyle::new(white, white);
        let black_style: ColorStyle = ColorStyle::new(black, black);    

        for y in 0..HEIGHT
        {
            for x in 0..WIDTH
            {
                let cell = self.get_px((y * HEIGHT) + x);
                let style = if cell == 0 { white_style } else { black_style };

                p.with_color(style, |printer| {
                    printer.print((x, y), " ");
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
                // for n in 0..4096
                // {
                //     // let curr = self.memory[n];
                //     if curr == 0
                //     {
                //         self.set(n, 1);
                //     }
                //     else
                //     {
                //         self.set(n, 0);
                //     }
                // }
            
                EventResult::Consumed(None)
            }
            _ => EventResult::Ignored
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 { XY { x: 48, y: 32 } }
}
