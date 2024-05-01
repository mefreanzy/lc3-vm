use crate::register::Registers;
use crate::instructions::OpCode;
use std::env;
use std::fs::File;
use termios::*;
use std::os::unix::io::FromRawFd;
use std::io::{self, BufReader, Read};
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
    let stdin_fd = 0;
    
    let termios = Termios::from_fd(stdin_fd).expect("Failed to get terminal attributes");
    let mut new_termios = termios.clone();

    new_termios.c_lflag &= !(ICANON | ECHO);

    tcsetattr(stdin_fd, TCSANOW, &mut new_termios).expect("Failed to set terminal attributes");

    let mut vm = VM::new();

    let args: Vec<String> = env.args().collect();
    if args.len() != 2
    {
        eprintln!("Usage: cargo run <program-file.obj>");
        return;
    }

    let filename = &args[1];
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };
    let mut reader = BufReader::new(file);

    let mut address = 0;
    loop 
    {
        match reader.read_u16::<BigEndian>()
        {
            Ok(instruction) {
                vm.write_memory(address, instruction);
                adress += 1;
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("End of file reached");
                } else {
                    eprintln!("Error reading file: {}", err);
                }
                break;
            }
        }
    }

    vm.run();

    tcsetattr(stdin_fd, TCSANOW, &termios).expect("Failed to restore terminal attributes");

}
