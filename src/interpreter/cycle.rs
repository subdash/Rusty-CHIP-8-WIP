use super::{Interpreter};

struct Nibbles(u8, u8, u8, u8);
impl Nibbles
{
    const fn new(code: u16) -> Nibbles
    {
        let first = ((code & 0xF000) >> 12) as u8;
        let second = ((code & 0x0F00) >> 8) as u8;
        let third = ((code & 0x00F0) >> 4) as u8;
        let fourth = (code & 0x000F) as u8;
        return Nibbles(first, second, third, fourth);
    }    
}

impl Interpreter
{
    pub fn fetch(&mut self)
    {
        // let pc = usize::from(self.pc);
        let opcode_first_half = self.memory[self.pc] as u16;
        let opcode_second_half = self.memory[self.pc + 1] as u16;
        self.op_code = (opcode_first_half) << 8 | opcode_second_half;
        self.next_instruction();
        self.log_globals();
    }

    pub fn decode_and_execute(&mut self)
    {
        // First 4 bits
        let as_components = Nibbles::new(self.op_code);
        self.x = ((self.op_code & 0x0F00) >> 8) as usize;
        self.y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.n = (self.op_code & 0x000F) as u8;
        self.nn = (self.op_code & 0x00FF) as u8;
        self.nnn = self.op_code & 0x0FFF;

        match as_components
        {
            Nibbles(0x0, 0x0, 0xE, 0x0) => self.clear_display(),
            Nibbles(0x0, 0x0, 0xE, 0xE) => self.return_from_subroutine(),
            Nibbles(0x1, _, _, _)       => self.jump(),
            Nibbles(0x2, _, _, _)       => self.call_subroutine(),
            Nibbles(0x3, _, _, _)       => self.skip_if_vx_eq_nn(),
            Nibbles(0x4, _, _, _)       => self.skip_if_vx_neq_nn(),            
            Nibbles(0x5, _, _, 0)       => self.skip_if_vx_eq_vy(),
            Nibbles(0x6, _, _, _)       => self.set_vx_reg(),
            Nibbles(0x7, _, _, _)       => self.add_vx_reg(),
            Nibbles(0x8, _, _, 0x0)     => self.set_vx_to_vy(),
            Nibbles(0x8, _, _, 0x1)     => self.set_vx_oreq_vy(),
            Nibbles(0x8, _, _, 0x2)     => self.set_vx_andeq_vy(),
            Nibbles(0x8, _, _, 0x3)     => self.set_vx_xoreq_vy(),
            Nibbles(0x8, _, _, 0x4)     => self.set_vx_addeq_vy(),
            Nibbles(0x8, _, _, 0x5)     => self.set_vx_subeq_vy(),
            Nibbles(0x8, _, _, 0x6)     => self.set_vx_rshift_vy(),
            Nibbles(0x8, _, _, 0x7)     => self.set_vx_eq_vy_sub_vx(),
            Nibbles(0x8, _, _, 0xE)     => self.set_vx_lshift_vy(),
            Nibbles(0x9, _, _, 0x0)     => self.skip_if_vx_neq_vy(),
            Nibbles(0xA, _, _, _)       => self.set_idx_reg(),
            Nibbles(0xB, _, _, _)       => self.set_pc_to_v0_plus_nnn(),
            Nibbles(0xC, _, _, _)       => self.rand(),
            Nibbles(0xD, _, _, _)       => self.draw_instruction(),
            Nibbles(0xF, _, 0x0, 0x7)   => self.set_vx_to_delay(),
            Nibbles(0xF, _, 0x0, 0xA)   => self.get_key(),
            Nibbles(0xF, _, 0x1, 0x5)   => self.set_delay_to_vx(),
            Nibbles(0xF, _, 0x1, 0x8)   => self.set_sound_to_vx(),
            Nibbles(0xF, _, 0x1, 0xE)   => self.add_vx_to_i(),
            Nibbles(0xF, _, 0x2, 0x9)   => self.font_char(),
            Nibbles(0xF, _, 0x3, 0x3)   => self.reg_dump(),
            Nibbles(0xF, _, 0x5, 0x5)   => self.bin_decimal_conversion(),
            Nibbles(0xF, _, 0x6, 0x5)   => self.reg_load(),
            _ => self.unknown_op_code(),
        }
    }
}
