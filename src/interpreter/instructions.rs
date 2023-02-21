use rand::Rng;
use super::{Interpreter, graphics::WIDTH, graphics::HEIGHT};

impl Interpreter
{
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
        self.pc = ret_value;
    }

    /// 0x1NNN: jump to regiister NNN
    pub fn jump(&mut self)
    {
        self.debug_log(format!("CALL:   jump"));
        let address = self.op_code & 0x0FFF;
        // NOT working since PC gets incremented after jump
        self.debug_log(format!("Setting program counter to {:#06x}", address));
        self.pc = address;
        // self.skip_inc = true;
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
        let nn = (self.op_code & 0x00FF) as u8;

        self.v[x] += nn;
    }

    /// 0x8XY0: VX = VY
    pub fn set_vx_to_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_to_vy"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.debug_log(format!("V[x], V[y]: {}, {}", self.v[x], self.v[y]));
        self.v[x] = self.v[y];
    }
    
    /// 0x8XY1: VX |= VY
    pub fn set_vx_oreq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_oreq_vy"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.debug_log(format!("V[x], V[y]: {}, {}", self.v[x], self.v[y]));
        self.v[x]|= self.v[y];
    }
    /// 0x8XY2: VX &= VY
    pub fn set_vx_andeq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_andeq_vy"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.debug_log(format!("V[x], V[y]: {}, {}", self.v[x], self.v[y]));
        self.v[x]&= self.v[y];
    }
    /// 0x8XY3: VX ^= VY
    pub fn set_vx_xoreq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_xoreq_vy"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.debug_log(format!("V[x], V[y]: {}, {}", self.v[x], self.v[y]));
        self.v[x] ^= self.v[y];
    }
    /// 0x8XY4: VX += VY
    pub fn set_vx_addeq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_addeq_vy"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.debug_log(format!("V[x], V[y]: {}, {}", self.v[x], self.v[y]));
        self.v[x] += self.v[y];
    }
    /// 0x8XY5: VX -= VY
    pub fn set_vx_subeq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_subeq_vy"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.debug_log(format!("V[x], V[y]: {}, {}", self.v[x], self.v[y]));
        self.v[x] -= self.v[y];
    }
    /// 0x8XY6: VX >>= VY
    pub fn set_vx_rshift_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_subeq_vy"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.debug_log(format!("V[x], V[y]: {}, {}", self.v[x], self.v[y]));
        self.v[x] >>= self.v[y];
    }
    /// 0x8XY7: VX = VY - VX
    pub fn set_vx_eq_vy_sub_vx(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_eq_vy_sub_vx"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.debug_log(format!("V[x], V[y]: {}, {}", self.v[x], self.v[y]));
        self.v[x] = self.v[y] - self.v[x];
    }
    /// 0x8XYE: VX <<= VY
    pub fn set_vx_lshift_vy(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_eq_vy_sub_vx"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        self.debug_log(format!("V[x], V[y]: {}, {}", self.v[x], self.v[y]));
        self.v[x] <<= self.v[y];
    }
    /// 0x9XY0: skip if VX does not equal VY
    pub fn skip_if_vx_neq_vy(&mut self)
    {
        self.debug_log(format!("CALL:   skip_if_vx_neq_vy"));
        let x: usize = ((self.op_code & 0x0F00) >> 8) as usize;
        let y: usize = ((self.op_code & 0x00F0) >> 4) as usize;
        self.debug_log(format!("V[x], V[y]: {}, {}", self.v[x], self.v[y]));
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
    }

    /// 0xCXNN: V[x] = rand() & nn
    pub fn rand(&mut self)
    {
        self.debug_log(format!("CALL:   rand"));
        let mut rng = rand::thread_rng();
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let nn = (self.op_code & 0x00FF) as u8;
        let rand: u8 = rng.gen_range(0..255);
        self.v[x] = rand & nn;
    }

    /// 0xDXYN: draw n rows at coordinates in vx/vy
    pub fn draw_instruction(&mut self)
    {
        self.debug_log(format!("CALL:   draw"));
        self.draw_flag = true;
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        let y = ((self.op_code & 0x00F0) >> 4) as usize;
        let n = (self.op_code & 0x000F) as usize;
        self.v[0xF] = 0;

        for byte in 0..n
        {
            let row = (self.v[y] as usize + byte) % HEIGHT;
            for bit in 0..8
            {
                let col = (self.v[x] as usize + bit) % WIDTH;
                let color = (self.memory[self.i as usize + byte] >> (7 - bit)) & 1;
                self.v[0x0F] |= color & self.pixels[row][col];
                self.pixels[row][col] ^= color;
            }
        }
    }

    /// 0xFX07: set vx to current value of delay timer
    pub fn set_vx_to_delay(&mut self)
    {
        self.debug_log(format!("CALL:   set_vx_to_delay"));
        panic!("Unimplemented: set_vx_to_delay");
    }

    /// 0xFX15: set delay timer to current value in vx
    pub fn set_delay_to_vx(&mut self)
    {
        self.debug_log(format!("CALL:   set_delay_to_vx"));
        panic!("Unimplemented: set_delay_to_vx");
    }

    /// 0xFX18: set sound timer to current value in vx
    pub fn set_sound_to_vx(&mut self)
    {
        self.debug_log(format!("CALL:   set_sound_to_vx"));
        panic!("Unimplemented: set_sound_to_vx");
    }

    /// 0xFX1E: add VX to index
    pub fn add_vx_to_i(&mut self)
    {
        self.debug_log(format!("CALL:   add_vx_to_i"));
        let x = ((self.op_code & 0x0F00) >> 8) as usize;
        self.i += self.v[x] as u16;
    }

    /// 0xFX0A: stop executing instructions until key is pressed
    pub fn get_key(&mut self)
    {
        self.debug_log(format!("CALL:   get_key"));
        /*
            - Decrement PC and don't increment again until key is pressed
            - If key is pressed while waiting for input, its hex value will be
              stored in VX and execution continues
        */
        // Use device query lib
        // https://docs.rs/device_query/latest/device_query/
        panic!("Unimplemented: get_key");
    }

    /// 0xFX29: set i to font character
    pub fn font_char(&mut self)
    {
        self.debug_log(format!("CALL:   font_char"));
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
        self.debug_log(format!("CALL:   bin_decimal_conversion"));
        /*
            Take number in VX (which is one byte) and convert it to three decimal digits,
            storing them in memory at the address in index register I.
        */
        panic!("Unimplemented: bin_decimal_conversion");
    }

    /// 0xFX55: store V0 to VX in memory starting at I
    pub fn reg_dump(&mut self)
    {
        self.debug_log(format!("CALL:   reg_dump"));
        let x = (self.op_code & 0x0F00) >> 8;
        let start_addr = self.i;
        self.debug_log(format!("x, i: {:#06x}, {:#06x}", x, start_addr));
        for reg in 0x0..x + 1
        {
            let addr = start_addr + reg;
            self.debug_log(format!("address, value: {:#06x}, {:#06x}", addr, self.v[reg as usize]));
            self.memory[addr as usize] = self.v[reg as usize];
        }
    }

    /// 0xFX65: fill V0 to VX from memory starting at I
    pub fn reg_load(&mut self)
    {
        self.debug_log(format!("CALL:   reg_load"));
        let x = (self.op_code & 0x0F00) >> 8;
        let start_addr = self.i;
        for reg in 0x0..x + 1
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
