/*

    Memory
    0x000-0x1FF - Chip 8 interpreter
    0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    0x200-0xFFF - Program ROM and work RAM
*/
use super::Interpreter;

impl Interpreter
{
    ///
    /// OPCODE
    ///
    pub fn get_op_code(&self) -> u16
    {
        self.op_code.unwrap()
    }

    pub fn set_op_code(&mut self, value: u16)
    {
        self.op_code = Some(value);
    }
    ///
    /// Stack
    ///
    pub fn push_stack(&mut self, value: u16)
    {
        self.inc_sp();
        self.stack.unwrap()[self.get_sp()] = value;        
    }

    pub fn pop_stack(&mut self) -> u16
    {
        let value = self.peek_stack();
        self.dec_sp();
        value
    }

    fn peek_stack(&mut self) -> u16
    {
        self.stack.unwrap()[self.get_sp()]
    }

    pub fn get_stack(&mut self) -> [u16; 16]
    {
        self.stack.unwrap()
    }

    pub fn get_sp(&self) -> usize
    {
        self.sp.unwrap()
    }

    pub fn inc_sp(&mut self)
    {
        let current_sp = self.get_sp();
        self.set_sp(current_sp + 1);
    }

    pub fn dec_sp(&mut self)
    {
        let current_sp = self.get_sp();
        self.set_sp(current_sp - 1);
    }

    fn set_sp(&mut self, val: usize)
    {
        self.sp = Some(val);
    }
    ///
    /// Index register
    ///
    pub fn get_i(&self) -> u16
    {
        self.i.unwrap()
    }

    pub fn set_i(&mut self, val: u16)
    {
        self.i = Some(val);
    }

    ///
    /// Variable registers
    ///
    pub fn get_v(&self) -> [u8; 16]
    {
        self.v.unwrap()
    }

    pub fn get_v_reg(&self, index: usize) -> u8
    {
        self.v.unwrap()[index]
    }

    pub fn set_v_reg(&mut self, index: usize, value: u8)
    {
        // We need to actually retrieve a reference to the struct field
        if let Some(ref mut v) = self.v {
            v[index] = value;
        }
    }
    ///
    /// Other memory, graphics
    ///
    pub fn get_memory_at_address(&self, index: usize) -> u8
    {
        self.memory.unwrap()[index]
    }

    pub fn set_memory_address(&mut self, index: usize, value: u8)
    {
        // We need to actually retrieve a reference to the struct field
        if let Some(ref mut mem) = self.memory {
            mem[index] = value;
        }
    }

    pub fn get_graphics_at_address(&self, index: usize) -> u8
    {
        self.graphics.unwrap()[index]
    }

    pub fn set_graphics_at_address(&mut self, index: usize, value: u8)
    {
        // We need to actually retrieve a reference to the struct field
        if let Some(ref mut gfx) = self.graphics {
            gfx[index] = value;
        }
    }
    ///
    /// Program counter
    ///
    pub fn get_pc(&self) -> u16
    {
        self.pc.unwrap()
    }

    pub fn set_pc(&mut self, val: u16)
    {
        self.pc = Some(val);
    }

    pub fn next_instruction(&mut self)
    {
        if let Some(skip_inc) = self.skip_inc
        {
            let current_pc = self.get_pc();
            if skip_inc == false
            {
                self.debug_log(format!("PC: {:#06x} -> {:#06x}", current_pc, current_pc + 2));
                self.set_pc(current_pc + 2);
            }
            else
            {
                self.skip_inc = Some(false);
                self.debug_log(format!("PC: {:#06x}", current_pc));
            }
        }
        
    }
    ///
    /// Timers
    ///
    fn set_delay_timer(&mut self, value: u8)
    {
        self.delay_timer = Some(value);
    }

    pub fn dec_delay_timer(&mut self)
    {
        let current_value = self.delay_timer.unwrap();
        if current_value == 0
        {
            self.set_delay_timer(59);
            return;
        }
        self.set_delay_timer(current_value - 1);
    }

    fn set_sound_timer(&mut self, value: u8)
    {
        self.sound_timer = Some(value);
    }

    pub fn dec_sound_timer(&mut self)
    {
        let current_value = self.sound_timer.unwrap();
        if current_value == 0
        {
            // TODO: play beep sound
            self.set_sound_timer(59);
            return;
        }
        self.set_sound_timer(current_value - 1);
    }
}
