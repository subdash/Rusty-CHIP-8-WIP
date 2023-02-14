use super::Interpreter;

impl Interpreter
{
    pub fn is_debug(&mut self) -> bool
    {
        self.debug.unwrap()
    }

    pub fn debug_log(&mut self, msg: String)
    {
        if !self.is_debug()
        {
            return;
        }
        println!("{}", msg);
    }

    pub fn log_globals(&mut self)
    {
        /*
            TODO: Expose separate debug logging functions to keep members private
        */
        let op_code = self.get_op_code();
        let sp = self.get_sp();
        let i = self.get_i();
        let v = self.get_v();
        let pc = self.get_pc();
        println!();
        self.debug_log(format!("OP_CODE: {:#06x}", op_code));
        self.debug_log(format!("SP: {:#06x}", sp));
        self.debug_log(format!("I: {:#06x}", i));
        self.debug_log(format!("V: {:?}", v));
        self.debug_log(format!("PC: {:#06x}", pc));
    }
}

