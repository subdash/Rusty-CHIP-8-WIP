use super::Interpreter;

impl Interpreter
{
    /// 0x00E0 - clear screen
    pub fn clear_display(&mut self)
    {
        self.debug_log(format!("CALL:   clear_display"));
        if self.is_debug()
        {
            return;
        }
        print!("{}[2J", 27 as char);
    }

    /// 0x00EE: return from subroutine
    pub fn return_from_subroutine(&mut self)
    {
        self.debug_log(format!("CALL:   return_from_subroutine"));
        let ret_value = self.pop_stack();
        self.set_pc(ret_value);
    }

    /// 0x1NNN: jump to regiister NNN
    pub fn jump(&mut self)
    {
        self.debug_log(format!("CALL:   jump"));
        let address = self.get_op_code() & 0x0FFF;
        // NOT working since PC gets incremented after jump
        self.debug_log(format!("Setting program counter to {:#06x}", address));
        if (address == self.get_pc())
        {
            self.debug_log(format!("Infinite jump"))
        }
        self.set_pc(address);
        self.skip_inc = Some(true);
    }

    /// 0x2NNN: execute subroutine at address NNN
    pub fn call_subroutine(&mut self)
    {
        self.debug_log(format!("CALL:   call_subroutine"));
        let pc = self.get_pc();
        let op_code = self.get_op_code();

        self.push_stack(pc);
        self.set_pc(op_code & 0x0FFF);
    }

    /// 0x3XNN: skip if VX equals NN
    pub fn skip_if_vx_eq_nn(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_eq_nn"));
        
        let x_reg: usize = ((self.get_op_code() & 0x0F00) >> 8).try_into().unwrap();
        let x = self.get_v_reg(x_reg);
        let nn: u8 = (self.get_op_code() & 0x00FF).try_into().unwrap();
        let pc = self.get_pc();
        self.debug_log(format!("V[x]: {:#06x}", x));
        self.debug_log(format!("NN: {:#06x}", nn));
        if x == nn
        {
            self.next_instruction();
            self.debug_log(format!("PC post-skip: {:#06x}", pc));
            // self.debug_log(format!("MEMORY: {:?}", MEMORY));
        }
    }

    /// 0x4XNN: skip if VX does not equal NN
    pub fn skip_if_vx_neq_nn(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_neq_nn"));
        let x_reg: usize = ((self.get_op_code() & 0x0F00) >> 8).try_into().unwrap();
        let x = self.get_v_reg(x_reg);
        let nn: u8 = (self.get_op_code() & 0x00FF).try_into().unwrap();
        if x != nn
        {
            self.next_instruction();
        }
    }

    /// 0x5XY0: skip if VX equals VY
    pub fn skip_if_vx_eq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_eq_vy"));
        let x_reg: usize = ((self.get_op_code() & 0x0F00) >> 8).try_into().unwrap();
        let y_reg: usize = ((self.get_op_code() & 0x00F0) >> 4).try_into().unwrap();
        // Set x and y coordinates to values in VX/VY
        let x = self.get_v_reg(x_reg);
        let y = self.get_v_reg(y_reg);
        if x == y
        {
            self.next_instruction();
        }
    }

    /// 0x6XNN: set register VX to NN
    pub fn set_vx_reg(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_reg"));
        let x = (self.get_op_code() & 0x0F00) >> 8;
        let nn: u8 = (self.get_op_code() & 0x00FF).try_into().unwrap();
        let address_usize = usize::from(x);
        let curr_v_reg = self.get_v_reg(address_usize);
        self.debug_log(format!("Current V[{:#04x}]: {:#04x}", x, curr_v_reg));
        self.debug_log(format!("Setting V reg {:#04x} to {:#04x}", address_usize, nn));
        
        self.set_v_reg(address_usize, nn);
    }

    /// 0x7XNN: add NN to VX
    pub fn add_vx_reg(&mut self)
    {
        self.debug_log(format!("CALL:   add_vx_reg"));
        let x = (self.get_op_code() & 0x000F) >> 12;
        let nn: u8 = (self.get_op_code() & 0x00FF).try_into().unwrap();
        let address_usize = usize::from(x);
        let curr_v_reg = self.get_v_reg(address_usize);
        self.set_v_reg(address_usize, curr_v_reg + nn);
    }

    /// 0x9XY0: skip if VX does not equal VY
    pub fn skip_if_vx_neq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_neq_vy"));
        let x_reg: usize = ((self.get_op_code() & 0x0F00) >> 8).try_into().unwrap();
        let y_reg: usize = ((self.get_op_code() & 0x00F0) >> 4).try_into().unwrap();
        // Set x and y coordinates to values in VX/VY
        let x = self.get_v_reg(x_reg);
        let y = self.get_v_reg(y_reg);
        if x != y
        {
            self.next_instruction();
        }
    }

    /// 0xANNN: set I to NNN
    pub fn set_idx_reg(&mut self)
    {
        self.debug_log(format!("CALL:   set_idx_reg"));
        let op_code = self.get_op_code();        
        self.set_i(op_code & 0x0FFF);
        self.next_instruction();
    }

    /// 0xDXYN: draw n rows at coordinates in vx/vy
    pub fn draw(&mut self)
    {
        let x_reg: usize = ((self.get_op_code() & 0x0F00) >> 8).try_into().unwrap();
        let y_reg: usize = ((self.get_op_code() & 0x00F0) >> 4).try_into().unwrap();
        // Set x and y coordinates to values in VX/VY
        let x: u16 = (self.get_v_reg(x_reg) & 63).try_into().unwrap();
        let y: u16 = (self.get_v_reg(y_reg) & 31).try_into().unwrap();
        // Sprite height (n rows to draw)
        let n = self.get_op_code() & 0x000F;
        // Clear flag register
        self.set_v_reg(0xF, 0);

        // for r in 0..n
        for row in 0..n
        { 
            let pixel_loc = usize::from(self.get_i() + row);
            let pixel = self.get_memory_at_address(pixel_loc);

            for bit_offset in 0..8
            {
                // Check if current pixel is on
                if (pixel & (0x80 >> row)) != 0
                {
                    // Check if pixel(x,y) is on
                    let pixel_at_coords_loc = usize::from(x + bit_offset + ((y + row) * 64));
                    let graphics_byte = self.get_graphics_at_address(pixel_at_coords_loc);
                    if graphics_byte == 1
                    {
                        self.set_v_reg(0xF, 1);
                    }
                    // XOR pixel value
                    self.set_graphics_at_address(pixel_at_coords_loc, graphics_byte ^ 1);
                }
            }
        }
        // render();
    }

    /// 0xFX07: set vx to current value of delay timer
    pub fn set_vx_to_delay(&mut self)
    {
        panic!("NOT YET IMPLEMENTED - set_vx_to_delay");
    }

    /// 0xFX15: set delay timer to current value in vx
    pub fn set_delay_to_vx(&mut self)
    {
        panic!("NOT YET IMPLEMENTED - set_delay_to_vx");
    }

    /// 0xFX18: set sound timer to current value in vx
    pub fn set_sound_to_vx(&mut self)
    {
        panic!("NOT YET IMPLEMENTED - set_sound_to_vx");
    }

    /// 0xFX1E: add VX to index
    pub fn add_vx_to_i(&mut self)
    {
        panic!("NOT YET IMPLEMENTED - add_vx_to_i");
    }

    /// 0xFX0A: stop executing instructions until key is pressed
    pub fn get_key(&mut self)
    {
        /*
            - Decrement PC and don't increment again until key is pressed
            - If key is pressed while waiting for input, its hex value will be
              stored in VX and execution continues
        */
        // Use device query lib
        // https://docs.rs/device_query/latest/device_query/
        panic!("NOT YET IMPLEMENTED - get_key");
    }

    /// 0xFX29: set i to font character
    pub fn font_char(&mut self)
    {
        /*
            I is set to address of hex character in VX, may be able to take last nibble
            of VX and use that as the character
        */
        let x_reg: usize = ((self.get_op_code() & 0x0F00) >> 8).try_into().unwrap();
        let vx = self.get_v_reg(x_reg);
        let last_nibble: u16 = (vx & 0x000F).try_into().unwrap();
        self.debug_log(format!("vx: {:#06x}, last_nibble: {:#06x}", vx, last_nibble));
        self.set_i(last_nibble);
        // TODO: make sure this one is working correctly, V[X] and last nibble are 0
    }

    /// 0xFX33: binary-coded decimal conversion
    pub fn bin_decimal_conversion(&mut self)
    {
        /*
            Take number in VX (which is one byte) and convert it to three decimal digits,
            storing them in memory at the address in index register I.
        */
        panic!("NOT YET IMPLEMENTED - bin_decimal_conversion");
    }

    pub fn unknown_op_code(&mut self)
    {
        panic!("Unknown OP code: {:#06x}", self.get_op_code());
    }
}
