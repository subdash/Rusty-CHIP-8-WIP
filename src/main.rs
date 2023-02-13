use std::{thread, time};

fn main()
{
    unsafe
    {
        initialize();
        let instruction: [u8; 2] = [0x00, 0x00];
        loop
        {
            fetch();
            decode();
            execute();
            dec_timers();
            // print_screen();
            let refresh_interval = time::Duration::from_millis(16);
            thread::sleep(refresh_interval);
        }
    }
}

/*

    Fetch/decode/execute

*/
fn fetch()
{
    // https://stackoverflow.com/a/50244328
    // Take bits of mem[pc], shift left by one byte, take bytes
    // of mem[pc+1] and OR them into 8 bits on right side
    unsafe
    {
        let pc = usize::from(PC);
        OP_CODE = (MEMORY[pc] as u16) << 8 | MEMORY[pc + 1] as u16;
    }
}
// TODO: rename test
#[cfg(test)]
mod tests {
    #[test]
    fn exploration()
    {
        let first: u8 = 0xA2;
        let second: u8 = 0xF0;
        let result: u16 = (first as u16) << 8 | second as u16;
        assert!(result == 0xA2F0);
    }
}
unsafe fn decode()
{
    // First 4 bits
    match OP_CODE & 0xF000
    {
        0x0000 =>
        {
            // Last 4 bits
            match OP_CODE & 0x000F
            {
                // 0x00E0: Clear screen
                0x0000 => clear_display(),
                0x000E =>
                // Return from subroutine
                {

                }
                other => panic!("Unknown OP code: {}", other),
            }
        }
        // 0x1NNN: Jump to register NNN
        0x1000 => jump(OP_CODE & 0x0FFF),
        // 0x2NNN: Execute subroutine at address NNN
        0x2000 => call_subroutine(),
        
        // 0x6XNN: Set register VX
        0x6000 => 
        {
            let x = (OP_CODE & 0x000F) >> 12;
            let nn = OP_CODE & 0x00FF;
            set_flag_reg(x, nn.try_into().unwrap());
        }
        // 0x7XNN: Add value to register VX
        0x7000 =>
        {
            let x = (OP_CODE & 0x000F) >> 12;
            let nn = OP_CODE & 0x00FF;
            add_flag_reg(x, nn.try_into().unwrap());
        }
        // 0xANNN: set I to NNN
        0xA000 =>
        {
            I = OP_CODE & 0x0FFF;
            PC += 2;
        }
        /*
            Draw: draw sprite N pixels tall from memory location I,
            at (VX, VY).

            All pixels in the sprite that are on in the sprite will flip the
            pixels on the screen drawn from left to right. If any pixels on
            the screen are turned off, VF flag register is set to 1, otherwise
            it is set to 0.
        */
        0xD000 =>
        {

        }
        other => panic!("Unknown OP code: {}", other),
    }
}

fn execute() {}
/*

    Timers

*/

unsafe fn call_subroutine()
{
    let sp = usize::from(SP);
    stack[sp] = PC;
    SP += 1;
    PC = OP_CODE & 0x0FFF;
}

unsafe fn jump(address: u16)
{
    PC = address;
}

unsafe fn set_flag_reg(address: u16, value: u8)
{
    let address_usize = usize::from(address);
    V[address_usize] = value;
}

unsafe fn add_flag_reg(address: u16, value: u8)
{
    let address_usize = usize::from(address);
    V[address_usize] = V[address_usize] + value;
}

unsafe fn dec_timers()
{
    dec_delay_timer();
    dec_sound_timer();
}

unsafe fn dec_delay_timer()
{
    if DELAY_TIMER == 0
    {
        DELAY_TIMER = 59;
    }
    DELAY_TIMER -= 1;
}
unsafe fn dec_sound_timer()
{
    if SOUND_TIMER == 0
    {
        // BEEP
        SOUND_TIMER = 59;
    }
    SOUND_TIMER -= 1;
}

/*

    Memory

    0x000-0x1FF - Chip 8 interpreter
    0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    0x200-0xFFF - Program ROM and work RAM
*/
const stack: [u16; 16] = [0; 16];
static mut SP: u8 = 0x0000;
// Index register
static mut I: u16 = 0x0000;
// Variable registers
const V: [u8; 16] = [0; 16];
static mut MEMORY: [u8; 4096] = [0; 4096];
// Program counter
static mut PC: u16 = 0x0200;
static mut DELAY_TIMER: u8 = 0x3c; // 60
static mut SOUND_TIMER: u8 = 0x3c; // 60
static mut OP_CODE: u16 = 0x0000;
static mut KEYPAD: [bool; 16] = [false; 16];
// 16 5 byte sprites
const sprites: [u8; 80] =
[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
/*

    Display

*/
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
type Row = [bool; WIDTH];
type Screen = [[bool; WIDTH]; HEIGHT];
static DISPLAY: Screen = [[true; WIDTH]; HEIGHT];
fn print_screen()
{
    clear_display();
    let whole_screen = screen_string();
    println!("{whole_screen}");   
}

fn screen_string() -> String
{
    DISPLAY.map(|row| { row_string(&row) })
        .join("\n")
}

fn row_string(row: &Row) -> String
{
    row
        .iter()
        .map(|val| { if *val == true { "▀" } else { " " } })
        .collect()
}

fn clear_display()
{
    print!("{}[2J", 27 as char);
}

fn v_flag() -> bool
{
    V[0xF] != 0
}

unsafe fn load_sprites()
{
    // Store sprites 0x00..0x50
    for n in 0..80
    {
        MEMORY[n] = sprites[n];
    }
}

unsafe fn initialize()
{
    load_sprites();
    clear_display();
}
