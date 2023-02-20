use std::unimplemented;
use super::{Interpreter, graphics::WIDTH, graphics::HEIGHT};

impl Interpreter
{
    /// 0x00E0 - clear screen
    pub fn clear_display(&mut self)
    {
        self.debug_log(format!("CALL:   clear_display"));
        self.draw_flag = true;
        for row in 0..HEIGHT
        {
            for col in 0..WIDTH
            {
                self.pixels[row][col] = 0;
            }
            
        }
    }

    /// 0x00EE: return from subroutine
    pub fn return_from_subroutine(&mut self)
    {
        self.debug_log(format!("CALL:   return_from_subroutine"));
        let ret_value = self.pop_stack();
        self.pc = ret_value;
    }

    /// 0x1NNN: jump to regiister NNN
    pub fn jump(&mut self)
    {
        self.debug_log(format!("CALL:   jump"));
        let address = self.op_code & 0x0FFF;
        // NOT working since PC gets incremented after jump
        self.debug_log(format!("Setting program counter to {:#06x}", address));
        if address == self.pc
        {
            self.debug_log(format!("Infinite jump"))
        }
        self.pc = address;
        self.skip_inc = true;
    }

    /// 0x2NNN: execute subroutine at address NNN
    pub fn call_subroutine(&mut self)
    {
        self.debug_log(format!("CALL:   call_subroutine"));

        self.push_stack(self.pc);
        self.pc = self.op_code & 0x0FFF;
    }

    /// 0x3XNN: skip if VX equals NN
    pub fn skip_if_vx_eq_nn(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_eq_nn"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let nn = (self.op_code & 0x00FF) as u8;
        if self.v[x] == nn
        {
            self.next_instruction();
        }
    }

    /// 0x4XNN: skip if VX does not equal NN
    pub fn skip_if_vx_neq_nn(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_neq_nn"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let nn = (self.op_code & 0x00FF) as u8;
        if self.v[x] != nn
        {
            self.next_instruction();
        }
    }

    /// 0x5XY0: skip if VX equals VY
    pub fn skip_if_vx_eq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_eq_vy"));
        let x = (self.op_code & 0x0F00 >> 8) as usize;
        let y = (self.op_code & 0x00F0 >> 4) as usize;
 
        if self.v[x] == self.v[y]
        {
            self.next_instruction();
        }
    }

    /// 0x6XNN: set register VX to NN
    pub fn set_vx_reg(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_reg"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let nn = (self.op_code & 0x00FF) as u8;

        self.v[x] = nn;
    }

    /// 0x7XNN: add NN to VX
    pub fn add_vx_reg(&mut self)
    {
        self.debug_log(format!("CALL:   add_vx_reg"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let nn: u8 = (self.op_code & 0x00FF).try_into().unwrap();

        self.v[x] += nn;
    }

    /// 0x8XY0: VX = VY
    pub fn set_vx_to_vy(&mut self)
    {
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.v[x] = self.v[y];
    }
    
    /// 0x8XY1: VX |= VY
    pub fn set_vx_oreq_vy(&mut self)
    {
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.v[x]|= self.v[y];
    }
    /// 0x8XY2: VX &= VY
    pub fn set_vx_andeq_vy(&mut self)
    {
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.v[x]&= self.v[y];
    }
    /// 0x8XY3: VX ^= VY
    pub fn set_vx_xoreq_vy(&mut self)
    {
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.v[x]&= self.v[y];
    }
    /// 0x8XY4: VX += VY
    pub fn set_vx_addeq_vy(&mut self)
    {
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.v[x]&= self.v[y];
    }
    /// 0x8XY5: VX -= VY
    pub fn set_vx_subeq_vy(&mut self)
    {
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.v[x]&= self.v[y];
    }
    /// 0x8XY6: VX >>= VY
    pub fn set_vx_rshift_vy(&mut self)
    {
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.v[x] >>= self.v[y];
    }
    /// 0x8XY7: VX = VY - VX
    pub fn set_vx_eq_vy_sub_vx(&mut self)
    {
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.v[x] >>= self.v[y];
    }
    /// 0x8XYE: VX <<= VY
    pub fn set_vx_lshift_vy(&mut self)
    {
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.v[x] <<= self.v[y];
    }
    /// 0x9XY0: skip if VX does not equal VY
    pub fn skip_if_vx_neq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_neq_vy"));
        let x: usize = ((self.op_code & 0x0F00) >> 8) as usize;
        let y: usize = ((self.op_code & 0x00F0) >> 4) as usize;

        if self.v[x] != self.v[y]
        {
            self.next_instruction();
        }
    }

    /// 0xANNN: set I to NNN
    pub fn set_idx_reg(&mut self)
    {
        self.debug_log(format!("CALL:   set_idx_reg"));
        self.i = self.op_code & 0x0FFF;
        self.next_instruction();
    }

    /// 0xDXYN: draw n rows at coordinates in vx/vy
    pub fn draw_instruction(&mut self)
    {
        // self.debug_log(format!("draw_instruction called"));
        // self.set_draw_flag(true);
        // let x_reg: usize = ((self.op_code & 0x0F00) >> 8).try_into().unwrap();
        // let y_reg: usize = ((self.op_code & 0x00F0) >> 4).try_into().unwrap();
        // // Set x and y coordinates to values in VX/VY
        // let x: u16 = (self.get_v_reg(x_reg) & 63).try_into().unwrap();
        // let y: u16 = (self.get_v_reg(y_reg) & 31).try_into().unwrap();
        // // Sprite height (n rows to draw)
        // let sprite_height = self.op_code & 0x000F; // N
        // // Clear flag register
        // self.set_v_reg(0xF, 0);

        // for row in 0..sprite_height
        // { 
        //     let pixel_loc = usize::from(self.get_i() + row);
        //     let pixel = self.get_px(pixel_loc);

        //     for bit_offset in 0..8
        //     {
        //         // Check if current pixel is on
        //         if (pixel & (0x80 >> row)) != 0
        //         {
        //             // Check if pixel(x,y) is on
        //             let pixel_at_coords_loc = usize::from(x + bit_offset + ((y + row) * 64));
        //             let graphics_byte = self.get_px(pixel_at_coords_loc);
        //             if graphics_byte == 1
        //             {
        //                 self.set_v_reg(0xF, 1);
        //             }
        //             // XOR pixel value
        //             self.set_px(pixel_at_coords_loc, graphics_byte ^ 1);
        //         }
        //     }
        // }
        // self.debug_log(format!("{:?}", self.pixels));
    }

    /// 0xFX07: set vx to current value of delay timer
    pub fn set_vx_to_delay(&mut self)
    {
        unimplemented!("set_vx_to_delay");
    }

    /// 0xFX15: set delay timer to current value in vx
    pub fn set_delay_to_vx(&mut self)
    {
        unimplemented!("set_delay_to_vx");
    }

    /// 0xFX18: set sound timer to current value in vx
    pub fn set_sound_to_vx(&mut self)
    {
        unimplemented!("set_sound_to_vx");
    }

    /// 0xFX1E: add VX to index
    pub fn add_vx_to_i(&mut self)
    {
        unimplemented!("add_vx_to_i");
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
        unimplemented!("get_key");
    }

    /// 0xFX29: set i to font character
    pub fn font_char(&mut self)
    {
        /*
            I is set to address of hex character in VX, may be able to take last nibble
            of VX and use that as the character
        */
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let last_nibble = (self.v[x] & 0x000F) as u16;
        self.debug_log(format!("vx: {:#06x}, last_nibble: {:#06x}", self.v[x], last_nibble));
        self.i = last_nibble;
        // TODO: make sure this one is working correctly, V[X] and last nibble are 0
    }

    /// 0xFX33: binary-coded decimal conversion
    pub fn bin_decimal_conversion(&mut self)
    {
        /*
            Take number in VX (which is one byte) and convert it to three decimal digits,
            storing them in memory at the address in index register I.
        */
        unimplemented!("bin_decimal_conversion");
    }

    pub fn unknown_op_code(&mut self)
    {
        panic!("Unknown OP code: {:#06x}", self.op_code);
    }
}
