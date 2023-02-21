use std::fs;
use std::io::Read;
use super::Interpreter;

impl Interpreter
{
    pub fn initialize(&mut self)
    {
        self.load_sprites();
        self.load_program();
    }

    fn load_sprites(&mut self)
    {
        // Store sprites 0x00..0x50
        for n in 0..80
        {
            self.memory[n] = SPRITES[n];
        }
    }
    
    fn load_program(&mut self)
    {
        /*
            TODO: read file name from cmd line args, use that value here
        */
        let byte_vec = get_file_as_byte_vec(
            // "src/Stars [Sergey Naydenov, 2010].ch8"
            "src/IBM Logo.ch8"
        );
        let vec_len = byte_vec.len();

        for n in 0..vec_len
        {
            self.memory[0x200 + n] = byte_vec[n];
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

pub const SPRITES: [u8; 80] =
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
