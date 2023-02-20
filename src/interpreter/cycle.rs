use super::{Interpreter};
// use std::{thread, time};

impl Interpreter
{
    pub fn fetch(&mut self)
    {
        // let pc = usize::from(self.pc);
        let opcode_first_half = self.memory[self.pc as usize] as u16;
        let opcode_second_half = self.memory[self.pc as usize + 1] as u16;
        self.op_code = (opcode_first_half) << 8 | opcode_second_half;
        self.next_instruction();
        self.log_globals();
    }

    pub fn decode_and_execute(&mut self)
    {
        // First 4 bits
        match self.op_code & 0xF000
        {
            0x0000 =>
            {
                // Last 4 bits
                match self.op_code & 0x000F
                {
                    // 0x00E0: Clear screen
                    0x0000 => self.clear_display(),
                    // 0x00EE: return from subroutine
                    0x000E => self.return_from_subroutine(),                
                    _ => self.unknown_op_code(),
                }
            }
            // 0x1NNN: Jump to register NNN
            0x1000 => self.jump(),
            // 0x2NNN: Execute subroutine at address NNN
            0x2000 => self.call_subroutine(),
            // 0x3XNN
            0x3000 => self.skip_if_vx_eq_nn(),
            // 0x4XNN
            0x4000 => self.skip_if_vx_neq_nn(),
            // 0x5XY0
            0x5000 => self.skip_if_vx_eq_vy(),
            // 0x6XNN: Set register VX
            0x6000 => self.set_vx_reg(),
            // 0x7XNN: Add value to register VX
            0x7000 => self.add_vx_reg(),
            0x8000 =>
            {
                // Last 4 bits
                match self.op_code & 0x000F
                {
                    // 0x8XY0
                    0x0000 => self.set_vx_to_vy(),
                    // 0x8XY1
                    0x0001 => self.set_vx_oreq_vy(),
                    // 0x8XY2
                    0x0002 => self.set_vx_andeq_vy(),
                    // 0x8XY3
                    0x0003 => self.set_vx_xoreq_vy(),
                    // 0x8XY4
                    0x0004 => self.set_vx_addeq_vy(),
                    // 0x8XY5
                    0x0005 => self.set_vx_subeq_vy(),
                    // 0x8XY6
                    0x0006 => self.set_vx_rshift_vy(),
                    // 0x8XY7
                    0x0007 => self.set_vx_eq_vy_sub_vx(),
                    // 0x8XYE
                    0x000E => self.set_vx_lshift_vy(),
                    _ => self.unknown_op_code(),
                }
            }
            // 0x9XY0
            0x9000 => self.skip_if_vx_neq_vy(),
            // 0xANNN: Set I to NNN
            0xA000 => self.set_idx_reg(),
            // 0xDXYN: Draw
            0xD000 => self.draw_instruction(),
            0xF000 =>
            {
                // Last 8 bits
                match self.op_code & 0x00FF
                {
                    // 0xFX07: Set VX to current value of delay timer
                    0x0007 => self.set_vx_to_delay(),
                    // 0xFX15: Set delay timer to current value in vx
                    0x0015 => self.set_delay_to_vx(),
                    // 0xFX18:
                    0x0018 => self.set_sound_to_vx(),
                    // 0xFX1E:
                    0x001E => self.add_vx_to_i(),
                    // 0xFX0A:
                    0x000A => self.get_key(),
                    // 0xFX29:
                    0x0029 => self.font_char(),
                    // 0xFX33:
                    0x0033 => self.bin_decimal_conversion(),
                    _ => self.unknown_op_code(),
                }
            }
            _ => self.unknown_op_code(),
        }
    }
}
