use rand::Rng;
use super::{Interpreter, graphics::WIDTH, graphics::HEIGHT};

impl Interpreter
{
    fn jump_to(&mut self, address: usize)
    {
        self.pc = address;
        self.skip_inc = true;
    }
    /// 0x00E0 - clear screen
    pub fn clear_display(&mut self)
    {
        self.debug_log(format!("CALL:   clear_display"));
        self.draw_flag = true;
        self.pixels = [[0; WIDTH]; HEIGHT];
    }

    /// 0x00EE: return from subroutine
    pub fn return_from_subroutine(&mut self)
    {
        self.debug_log(format!("CALL:   return_from_subroutine"));
        let ret_value = self.pop_stack();
        self.debug_log(format!("Setting program counter to {:#06x}", ret_value));
        self.jump_to(usize::from(ret_value));
    }

    /// 0x1NNN: jump to NNN
    pub fn jump(&mut self)
    {
        self.debug_log(format!("CALL:   jump"));
        self.debug_log(format!("Setting program counter to {:#06x}", self.nnn));
        self.jump_to(usize::from(self.nnn));
    }

    /// 0x2NNN: execute subroutine at address NNN
    pub fn call_subroutine(&mut self)
    {
        self.debug_log(format!("CALL:   call_subroutine"));
        self.debug_log(format!("Setting program counter to {:#06x}", self.nnn));
        self.push_stack(self.pc as u16);
        self.jump_to(usize::from(self.nnn));
    }

    /// 0x3XNN: skip if VX equals NN
    pub fn skip_if_vx_eq_nn(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_eq_nn"));
        if self.v[self.x] == self.nn
        {
            self.next_instruction();
        }
    }

    /// 0x4XNN: skip if VX does not equal NN
    pub fn skip_if_vx_neq_nn(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_neq_nn"));
        if self.v[self.x] != self.nn
        {
            self.next_instruction();
        }
    }

    /// 0x5XY0: skip if VX equals VY
    pub fn skip_if_vx_eq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_eq_vy"));
        if self.v[self.x] == self.v[self.y]
        {
            self.next_instruction();
        }
    }

    /// 0x6XNN: set register VX to NN
    pub fn set_vx_reg(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_reg"));
        self.debug_log(format!("setting v[{:#04x}] to {:#04x}", self.x, self.nn));
        self.v[self.x] = self.nn;
        assert_eq!(self.v[self.x], self.nn);
    }

    /// 0x7XNN: add NN to VX
    pub fn add_vx_reg(&mut self)
    {
        self.debug_log(format!("CALL:   add_vx_reg"));
        self.v[self.x] = self.v[self.x].wrapping_add(self.nn);
    }

    /// 0x8XY0: VX = VY
    pub fn set_vx_to_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_to_vy"));
        self.v[self.x] = self.v[self.y];
    }
    
    /// 0x8XY1: VX |= VY
    pub fn set_vx_oreq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_oreq_vy"));
        self.v[self.x] |= self.v[self.y];
    }
    /// 0x8XY2: VX &= VY
    pub fn set_vx_andeq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_andeq_vy"));
        self.v[self.x] &= self.v[self.y];
    }
    /// 0x8XY3: VX ^= VY
    pub fn set_vx_xoreq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_xoreq_vy"));
        self.v[self.x] ^= self.v[self.y];
    }
    /// 0x8XY4: VX += VY
    pub fn set_vx_addeq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_addeq_vy"));
        self.v[self.x] = self.v[self.x].wrapping_add(self.v[self.y]);
    }
    /// 0x8XY5: VX -= VY
    pub fn set_vx_subeq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_subeq_vy"));
        self.v[self.x] = self.v[self.x].wrapping_sub(self.v[self.y]);
    }
    /// 0x8XY6: VX >>= VY
    pub fn set_vx_rshift_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_rshift_vy"));
        self.v[self.x] = self.v[self.x].wrapping_shr(self.v[self.y] as u32);
    }
    /// 0x8XY7: VX = VY - VX
    pub fn set_vx_eq_vy_sub_vx(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_eq_vy_sub_vx"));
        self.v[self.x] = self.v[self.y].wrapping_sub(self.v[self.x]);
    }
    /// 0x8XYE: VX <<= VY
    pub fn set_vx_lshift_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_lshift_vy"));
        self.v[self.x] = self.v[self.x].wrapping_shl(self.v[self.y] as u32);
    }
    /// 0x9XY0: skip if VX does not equal VY
    pub fn skip_if_vx_neq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_neq_vy"));
        if self.v[self.x] != self.v[self.y]
        {
            self.next_instruction();
        }
    }

    /// 0xANNN: set I to NNN
    pub fn set_idx_reg(&mut self)
    {
        self.debug_log(format!("CALL:   set_idx_reg"));
        self.i = self.nnn;
    }

    /// 0xBNNN: PC = V[0] + nnn
    pub fn set_pc_to_v0_plus_nnn(&mut self)
    {
        self.debug_log(format!("CALL:   set_idx_reg"));
        self.pc = usize::from(self.v[0x0 as usize] as u16 + self.nnn);
    }

    /// 0xCXNN: V[x] = rand() & nn
    pub fn rand(&mut self)
    {
        self.debug_log(format!("CALL:   rand"));
        let mut rng = rand::thread_rng();
        let rand: u8 = rng.gen_range(0..255);
        self.v[self.x] = rand & self.nn;
    }

    /// 0xDXYN: draw n rows at coordinates in vx/vy
    pub fn draw_instruction(&mut self)
    {
        self.debug_log(format!("CALL:   draw"));
        self.draw_flag = true;
        self.v[0xF] = 0;

        for byte in 0..self.n
        {
            let row = (self.v[self.y] + byte) % HEIGHT as u8;
            for bit in 0..8
            {
                let col = (self.v[self.x] as usize + bit) % WIDTH;
                let color = (self.memory[self.i as usize + byte as usize] >> (7 - bit)) & 1;
                self.v[0x0F] |= color & self.pixels[row as usize][col];
                self.pixels[row as usize][col] ^= color;
            }
        }
    }

    /// 0xEX9E: skip if key in vx is pressed
    pub fn skip_if_key_pressed(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_key_pressed"));
        if self.keypad.as_mut().unwrap().keys[self.v[self.x] as usize] == 1
        {
            self.next_instruction();
        }
    }
    /// 0xEXA1: skip if key in vx is not pressed
    pub fn skip_if_key_not_pressed(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_key_not_pressed"));
        if self.keypad.as_mut().unwrap().keys[self.v[self.x] as usize] == 0
        {
            self.next_instruction();
        }
    }

    /// 0xFX0A: wait for keypress and store in vx
    pub fn wait_for_keypress(&mut self)
    {
        self.debug_log(format!("CALL:   wait_for_keypress"));
        let mut key_pressed = false;
        for (i, key) in self.keypad.as_mut().unwrap().keys.iter().enumerate()
        {
            if *key == 1
            {
                self.v[self.x] = i as u8;
                key_pressed = true;
            }
        }
        if !key_pressed
        {
            self.pc -= 2;
        }
    }

    /// 0xFX07: set vx to current value of delay timer
    pub fn set_vx_to_delay(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_to_delay"));
        self.v[self.x] = self.delay_timer;
    }

    /// 0xFX15: set delay timer to current value in vx
    pub fn set_delay_to_vx(&mut self)
    {
        self.debug_log(format!("CALL:   set_delay_to_vx"));
        self.delay_timer = self.v[self.x];
    }

    /// 0xFX18: set sound timer to current value in vx
    pub fn set_sound_to_vx(&mut self)
    {
        self.debug_log(format!("CALL:   set_sound_to_vx"));
        self.sound_timer = self.v[self.x];
    }

    /// 0xFX1E: add VX to index
    pub fn add_vx_to_i(&mut self)
    {
        self.debug_log(format!("CALL:   add_vx_to_i"));
        self.i += self.v[self.x] as u16;
    }

    /// 0xFX29: set i to font character
    pub fn font_char(&mut self)
    {
        self.debug_log(format!("CALL:   font_char"));
        /*
            I is set to address of hex character in VX, may be able to take last nibble
            of VX and use that as the character
        */
        let last_nibble = self.v[self.x] & 0x000F;
        self.debug_log(format!("vx: {:#06x}, last_nibble: {:#06x}", self.v[self.x], last_nibble));
        self.i = last_nibble as u16;
        // TODO: make sure this one is working correctly, V[X] and last nibble are 0
    }

    /// 0xFX33: binary-coded decimal conversion
    pub fn bin_decimal_conversion(&mut self)
    {
        /*
            Take number in VX (which is one byte) and convert it to three decimal digits,
            storing them in memory at the address in index register I.
        */
        self.debug_log(format!("CALL:   bin_decimal_conversion"));
        self.memory[self.i as usize] = self.v[self.x as usize] / 100;
        self.memory[self.i as usize + 1] = (self.v[self.x as usize] % 100) / 10;
        self.memory[self.i as usize + 2] = self.v[self.x as usize] % 10;
    }

    /// 0xFX55: store V0 to VX in memory starting at I
    pub fn reg_dump(&mut self)
    {
        self.debug_log(format!("CALL:   reg_dump"));
        let start_addr = self.i;
        self.debug_log(format!("x, i: {:#06x}, {:#06x}", self.x, start_addr));
        for reg in 0x0..self.x + 1
        {
            let addr = start_addr as usize + reg;
            self.debug_log(format!("address, value: {:#06x}, {:#06x}", addr, self.v[reg as usize]));
            self.memory[addr as usize] = self.v[reg as usize];
        }
    }

    /// 0xFX65: fill V0 to VX from memory starting at I
    pub fn reg_load(&mut self)
    {
        self.debug_log(format!("CALL:   reg_load"));
        let start_addr = self.i as usize;
        for reg in 0x0..self.x + 1
        {
            let addr = start_addr + reg;
            self.v[reg as usize] = self.memory[addr as usize];
        }
    }

    pub fn unknown_op_code(&mut self)
    {
        self.debug_log(format!("Unknown OP code: {:#06x}", self.op_code));
        panic!("Unknown OP code: {:#06x}", self.op_code);
    }
}
