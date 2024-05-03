use crate::{register::Registers, instructions::OpCode};

const MEMORY_SIZE: usize = 1 << 16;
pub struct VM 
{
    pub memory: [u16; MEMORY_SIZE],
    pub reg: Registers,
}

impl VM 
{
    pub fn new() -> Self
    {
        let memory = [0; MEMORY_SIZE];
        let reg = Registers::new(0);

        VM { memory, reg }
    }

    pub fn read_memory(&self, address: u16) -> u16
    {
        self.memory[address as usize]
    }

    pub fn write_memory(&mut self, address: u16, data: u16)
    {
        self.memory[address as usize] = data;
    }

    fn run(&mut self)
    {
        loop {
            let pc = self.reg.pc;
            let instruction = self.memory[pc as usize];
            let opcode = OpCode::from_u16(instruction).expect("Invalid opcode.");
            opcode.execute(self, instruction);
            self.reg.pc += 1;
        }
    }
}


