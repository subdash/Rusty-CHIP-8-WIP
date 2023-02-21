use self::graphics::{ HEIGHT, WIDTH };

pub struct Interpreter
{
    stack: [u16; 16],
    sp: usize,
    i: u16,
    v: [u8; 16],
    memory: [u8; 4096],
    pixels: [[u8; WIDTH]; HEIGHT],
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    op_code: u16,
    keypad: [bool; 16],
    debug: bool,
    skip_inc: bool,
    draw_flag: bool,
}

impl Interpreter
{
    pub const fn new() -> Interpreter
    {
        Interpreter
        {
            stack: [0; 16],
            sp: 0x0000,
            i: 0x0000,
            v: [0; 16],
            memory: [0; 4096],
            pixels: [[0; WIDTH]; HEIGHT],
            // Program counter
            pc: 0x0200,
            delay_timer: 0x3c, // 0
            sound_timer: 0x3c, // 0
            op_code: 0x0000,
            keypad: [false; 16],
            /*
                TODO: read debug flag from cmd line args, use that value here
            */
            debug: true, // set to true when debugging
            skip_inc: false,
            draw_flag: false,
        }
    }
}

mod cycle;
mod graphics;
mod instructions;
mod setup;
mod state;
mod util;
