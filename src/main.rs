use std::{thread, time};

use crate::interpreter::Interpreter;

mod lib;
mod interpreter;

fn main()
{
    let mut interpreter = Interpreter::new();
    interpreter.initialize();

    let refresh_millis = if interpreter.is_debug() { 100 } else { 16 };
    let refresh_interval = time::Duration::from_millis(refresh_millis);

    loop
    {
        interpreter.fetch();
        interpreter.decode_and_execute();
        interpreter.dec_delay_timer();
        interpreter.dec_sound_timer();
        // render();
        thread::sleep(refresh_interval);
    }    
}
