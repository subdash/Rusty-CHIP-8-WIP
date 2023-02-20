use super::Interpreter;

impl Interpreter
{
    pub fn debug_log(&mut self, msg: String)
    {
        if self.debug
        {
            println!("{}", msg);
        }
    }

    pub fn log_globals(&mut self)
    {
        /*
            TODO: Expose separate debug logging functions to keep members private
        */
        let v_string = self.v_string();
        let stack_string = self.stack_string(); 
        self.debug_log(format!("\nOP_CODE: {:#06x}", self.op_code));
        self.debug_log(format!("I_REG:   {:#06x}", self.i));
        self.debug_log(format!("PC:      {:#06x}", self.pc));
        self.debug_log(format!("{stack_string}"));
        self.debug_log(format!("{v_string}\n"));
    }

    pub fn stack_string(&mut self) -> String
    {
        let sp = self.sp;
        let s = self.stack;
        format!("\
___STACK___
SP:     {:#04x}
INDEX:  0x00 0x01 0x02 0x03 0x04 0x05 0x06 0x07 0x08 0x09 0x0A 0x0B 0x0C 0x0D 0x0E 0x0F
STACK:  {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x}",
sp, s[0x0], s[0x1], s[0x2], s[0x3], s[0x4], s[0x5], s[0x6], s[0x7], s[0x8], s[0x9], s[0xa], s[0xb], s[0xc], s[0xd], s[0xe], s[0xf])
    }

    pub fn v_string(&mut self) -> String
    {
        let v = self.v;
        format!("\
___V_REG___
V_REG:  0x00 0x01 0x02 0x03 0x04 0x05 0x06 0x07 0x08 0x09 0x0A 0x0B 0x0C 0x0D 0x0E 0x0F
VALUE:  {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x}",
v[0x0], v[0x1], v[0x2], v[0x3], v[0x4], v[0x5], v[0x6], v[0x7], v[0x8], v[0x9], v[0xa], v[0xb], v[0xc], v[0xd], v[0xe], v[0xf])
    }
}

