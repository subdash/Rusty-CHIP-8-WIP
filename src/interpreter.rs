pub struct Interpreter
{
    stack: Option<[u16; 16]>,
    sp: Option<usize>,
    i: Option<u16>,
    v: Option<[u8; 16]>,
    memory: Option<[u8; 4096]>,
    pixels: [u8; 2048],
    pc: Option<u16>,
    delay_timer: Option<u8>,
    sound_timer: Option<u8>,
    op_code: Option<u16>,
    keypad: Option<[bool; 16]>,
    debug: Option<bool>,
    skip_inc: Option<bool>,
    draw_flag: Option<bool>
}

impl Interpreter
{
    pub const fn new() -> Interpreter
    {
        Interpreter
        {
            stack: Some([0; 16]),
            sp: Some(0x0000),
            i: Some(0x0000),
            v: Some([0; 16]),
            memory: Some([0; 4096]),
            pixels: [0; 2048],
            // Program counter
            pc: Some(0x0200),
            delay_timer: Some(0x3c), // 60
            sound_timer: Some(0x3c), // 60
            op_code: Some(0x0000),
            keypad: Some([false; 16]),
            /*
                TODO: read debug flag from cmd line args, use that value here
            */
            debug: Some(false), // set to true when debugging
            skip_inc: Some(false),
            draw_flag: Some(false),
        }
    }
}

mod cycle;
mod graphics;
mod instructions;
mod setup;
mod state;
mod util;
