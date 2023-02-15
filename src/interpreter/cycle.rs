use super::Interpreter;

impl Interpreter
{
    pub fn fetch(&mut self)
    {
        // https://stackoverflow.com/a/50244328
        // Take bits of mem[pc], shift left by one byte, take bytes
        // of mem[pc+1] and OR them into 8 bits on right side
    
        let pc = usize::from(self.get_pc());
        let opcode_first_half = self.get_memory_at_address(pc);
        let opcode_second_half = self.get_memory_at_address(pc + 1);
        let new_op_code = (opcode_first_half as u16) << 8 | opcode_second_half as u16;
        self.set_op_code(new_op_code);
        self.next_instruction();
        self.log_globals();
    }

    pub fn decode_and_execute(&mut self)
    {
        // First 4 bits
        match self.get_op_code() & 0xF000
        {
            0x0000 =>
            {
                // Last 4 bits
                match self.get_op_code() & 0x000F
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
            // 0x9XY0
            0x9000 => self.skip_if_vx_neq_vy(),
            // 0xANNN: Set I to NNN
            0xA000 => self.set_idx_reg(),
            // 0xDXYN: Draw
            0xD000 => self.draw(),
            0xF000 =>
            {
                // Last 8 bits
                match self.get_op_code() & 0x00FF
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
