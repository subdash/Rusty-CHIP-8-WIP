use cursive::views::{ Dialog, LinearLayout, Panel };
use crate::interpreter::Interpreter;
use std::{thread, time};

mod interpreter;

fn main()
{
    let mut interpreter = Interpreter::new();
    interpreter.initialize();

    let mut siv = cursive::default();
    siv.set_fps(1);
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(
        Dialog::new()
            .title("CHIP-8")
            .content(LinearLayout::vertical().child(Panel::new(interpreter)))
    );

    siv.run();

    let refresh_millis = if interpreter.is_debug() { 100 } else { 16 };
    let refresh_interval = time::Duration::from_millis(refresh_millis);

    loop
    {
        interpreter.fetch();
        interpreter.decode_and_execute();
        interpreter.dec_delay_timer();
        interpreter.dec_sound_timer();
        // interpreter.render();
        interpreter.set_draw_flag(false);
        thread::sleep(refresh_interval);
    }
}
