use std::fs;
use std::io::Read;
use std::{thread, time};

const DEBUG: bool = true;
fn debug_log(msg: String)
{
    if !DEBUG
    {
        return;
    }
    println!("{}", msg);
}

unsafe fn log_globals()
{
    println!();
    debug_log(format!("OP_CODE: {:#06x}", OP_CODE));
    debug_log(format!("SP: {:#06x}", SP));
    debug_log(format!("I: {:#06x}", I));
    debug_log(format!("V: {:?}", V));
    debug_log(format!("PC: {:#06x}", PC));
}

fn main()
{
    unsafe
    {
        initialize();
        load_program();
        let refresh_millis = if DEBUG { 100 } else { 16 };
        let refresh_interval = time::Duration::from_millis(refresh_millis);
        loop
        {
            fetch();
            log_globals();
            decode_and_execute();
            dec_timers();
            render();
            thread::sleep(refresh_interval);
            if PC > 1500 { break };
        }
    }
}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = fs::File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

unsafe fn load_program()
{
    let mut byte_vec = get_file_as_byte_vec("src/bc_test.ch8");
    let vec_len = byte_vec.len();
    for n in 0..vec_len
    {
        MEMORY[0x200 + n] = byte_vec[n];
    }
    // debug_log(format!("MEMORY: {:?}", MEMORY));
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
        debug_log(format!("MEMORY[PC] {:#06x}", MEMORY[pc]));
        debug_log(format!("MEMORY[PC + 1] {:#06x}", MEMORY[pc + 1]));
        debug_log(format!("PC BEFORE FETCH: {:#06x}", PC));
        OP_CODE = (MEMORY[pc] as u16) << 8 | MEMORY[pc + 1] as u16;
        PC += 2;
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
unsafe fn decode_and_execute()
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
                // 0x00EE: return from subroutine
                0x000E => return_from_subroutine(),                
                other => unknown_op_code(),
            }
        }
        // 0x1NNN: Jump to register NNN
        0x1000 => jump(),
        // 0x2NNN: Execute subroutine at address NNN
        0x2000 => call_subroutine(),
        // 0x3XNN
        0x3000 => skip_if_vx_eq_nn(),
        // 0x4XNN
        0x4000 => skip_if_vx_neq_nn(),
        // 0x5XY0
        0x5000 => skip_if_vx_eq_vy(),
        // 0x6XNN: Set register VX
        0x6000 => set_vx_reg(),
        // 0x7XNN: Add value to register VX
        0x7000 => add_vx_reg(),
        // 0x9XY0
        0x9000 => skip_if_vx_neq_vy(),
        // 0xANNN: set I to NNN
        0xA000 => set_idx_reg(),
        // 0xDXYN: draw
        0xD000 => draw(),
        other => unknown_op_code(),
    }
}

unsafe fn unknown_op_code()
{
    panic!("Unknown OP code: {:#06x}", OP_CODE);
}
/*

    Timers

*/
unsafe fn return_from_subroutine()
{
    debug_log(format!("return_from_subroutine"));
    let sp = usize::from(SP);
    let ret_value = stack[sp];
    PC = ret_value;
    SP -= 1;
}
unsafe fn call_subroutine()
{
    debug_log(format!("call_subroutine"));
    let sp = usize::from(SP);
    stack[sp] = PC;
    SP += 1;
    PC = OP_CODE & 0x0FFF;
}

unsafe fn draw()
{
    let x_reg: usize = ((OP_CODE & 0x0F00) >> 8).try_into().unwrap();
    let y_reg: usize = ((OP_CODE & 0x00F0) >> 4).try_into().unwrap();
    // Set x and y coordinates to values in VX/VY
    let x = V[x_reg] & 63;
    let y = V[y_reg] & 31;
    // Sprite height (n rows to draw)
    let n = OP_CODE & 0x000F;
    // Clear flag register
    V[0xF] = 0;

    for r in 0..n
    {
        let row: u8 = r.try_into().unwrap();     
        let pixel_loc = usize::from(I + r);
        let pixel = MEMORY[pixel_loc];

        for bit_offset in 0..8
        {
            // Check if current pixel is on
            if (pixel & (0x80 >> row)) != 0
            {
                // Check if pixel(x,y) is on
                let pixel_at_coords_loc = usize::from(x + bit_offset + ((y + row) * 64));
                if GRAPHICS[pixel_at_coords_loc] == 1
                {
                    V[0xF] = 1;
                }
                // XOR pixel value 
                GRAPHICS[pixel_at_coords_loc] ^= 1;
            }
        }
    }
}

unsafe fn jump()
{
    debug_log(format!("jump"));
    let address = OP_CODE & 0x0FFF;
    PC = address;
}

unsafe fn set_vx_reg()
{
    debug_log(format!("set_vx_reg"));
    let x = (OP_CODE & 0x0F00) >> 8;
    let nn: u8 = (OP_CODE & 0x00FF).try_into().unwrap();
    let address_usize = usize::from(x);
    debug_log(format!("x: {:#06x}", address_usize));
    debug_log(format!("nn: {:#06x}", nn));
    debug_log(format!("v[x]: {:#06x}", V[address_usize]));
    V[address_usize] = nn;
}

unsafe fn add_vx_reg()
{
    debug_log(format!("add_vx_reg"));
    let x = (OP_CODE & 0x000F) >> 12;
    let nn: u8 = (OP_CODE & 0x00FF).try_into().unwrap();
    let address_usize = usize::from(x);
    V[address_usize] = V[address_usize] + nn;
}

unsafe fn set_idx_reg()
{
    debug_log(format!("set_idx_reg"));
    I = OP_CODE & 0x0FFF;
    PC += 2;
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
static mut V: [u8; 16] = [0; 16];
static mut MEMORY: [u8; 4096] = [0; 4096];
static mut GRAPHICS: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
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
// type Row = [bool; WIDTH];
// type Screen = [[bool; WIDTH]; HEIGHT];
// static DISPLAY: Screen = [[true; WIDTH]; HEIGHT];
unsafe fn render()
{
    if DEBUG
    {
        return;
    }

    for pixel in 0..2048
    {
        if pixel % WIDTH == 0
        {
            println!();
        }
        if GRAPHICS[pixel] == 0
        {
            print!(" ");
        }
        else
        {
            print!("▀");
        }
    }
}

unsafe fn clear_display()
{
    debug_log(format!("clear_display"));
    if DEBUG
    {
        return;
    }
    print!("{}[2J", 27 as char);
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
}

unsafe fn skip_if_vx_eq_nn()
{
    debug_log(format!("skip_if_vx_eq_nn"));
    
    let x_reg: usize = ((OP_CODE & 0x0F00) >> 8).try_into().unwrap();
    let x = V[x_reg];
    let nn: u8 = (OP_CODE & 0x00FF).try_into().unwrap();
    debug_log(format!("V[x]: {:#06x}", x));
    debug_log(format!("NN: {:#06x}", nn));
    if x == nn
    {
        PC += 2;
        debug_log(format!("PC post-skip: {:#06x}", PC));
        // debug_log(format!("MEMORY: {:?}", MEMORY));
    }
}

unsafe fn skip_if_vx_neq_nn()
{
    debug_log(format!("skip_if_vx_neq_nn"));
    let x_reg: usize = ((OP_CODE & 0x0F00) >> 8).try_into().unwrap();
    let x = V[x_reg];
    let nn: u8 = (OP_CODE & 0x00FF).try_into().unwrap();
    if x != nn
    {
        PC += 2;
    }
}

unsafe fn skip_if_vx_eq_vy()
{
    debug_log(format!("skip_if_vx_eq_vy"));
    let x_reg: usize = ((OP_CODE & 0x0F00) >> 8).try_into().unwrap();
    let y_reg: usize = ((OP_CODE & 0x00F0) >> 4).try_into().unwrap();
    // Set x and y coordinates to values in VX/VY
    let x = V[x_reg];
    let y = V[y_reg];
    if x == y
    {
        PC += 2;
    }
}

unsafe fn skip_if_vx_neq_vy()
{
    debug_log(format!("skip_if_vx_neq_vy"));
    let x_reg: usize = ((OP_CODE & 0x0F00) >> 8).try_into().unwrap();
    let y_reg: usize = ((OP_CODE & 0x00F0) >> 4).try_into().unwrap();
    // Set x and y coordinates to values in VX/VY
    let x = V[x_reg];
    let y = V[y_reg];
    if x != y
    {
        PC += 2;
    }
}
