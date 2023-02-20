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
    /// Stack
    ///
    pub fn push_stack(&mut self, value: u16)
    {
        self.stack[self.sp] = value;
        self.inc_sp();
    }

    pub fn pop_stack(&mut self) -> u16
    {
        self.dec_sp();
        self.peek_stack()
    }

    fn peek_stack(&mut self) -> u16
    {
        self.stack[self.sp]
    }

    pub fn inc_sp(&mut self)
    {
        self.sp += 1;
    }

    pub fn dec_sp(&mut self)
    {
        self.sp -= 1;
    }

    pub fn next_instruction(&mut self)
    {
        if self.skip_inc == false
        {
            self.debug_log(format!("PC: {:#06x} -> {:#06x}", self.pc, self.pc + 2));
            self.pc += 2;
        }
        else
        {
            self.skip_inc = false;
            self.debug_log(format!("PC: {:#06x}", self.pc));
        }        
    }
    ///
    /// Timers
    ///
    pub fn dec_delay_timer(&mut self)
    {
        let current_value = self.delay_timer;
        if current_value == 0
        {
            self.delay_timer = 59;
            return;
        }
        self.delay_timer -= 1;
    }

    pub fn dec_sound_timer(&mut self)
    {
        let current_value = self.sound_timer;
        if current_value == 0
        {
            // TODO: play beep sound
            self.sound_timer = 59;
            return;
        }
        self.sound_timer -= 1;
    }
}
