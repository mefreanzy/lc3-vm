use crate::register::Registers;
use crate::instructions::OpCode
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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


fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 
    {
        eprintln!("Usage cargo run <program-file.obj>");
        std::process::exit(1);
    }

    let fname = &args[1];
    let f = File::open(fname).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut vm = VM::new();


    for line in reader.lines()
    {
        if let Ok(line) = line
        {
            let opcode = OpCode::from_u16(instruction).expect("Invalid opcode.");
            opcode.execute(&mut vm, instruction);
        } else {
            eprintln!("Invalid instruction: {}", line)
        }
    }

    vm.run();
}
