use std::io::{Read, Write};
use std::process;
use crate::lc3::VM;

#[derive(Debug)]
pub enum OpCode
{
    BR = 0,
    ADD,
    LD,
    ST,
    JSR,
    AND,
    LDR,
    STR,
    RTI,
    NOT,
    LDI,
    STI,
    JMP,
    RES,
    LEA,
    TRAP,
}

impl OpCode
{
    pub fn from_u16(instruction: u16) -> Option<Self>
    {
        match instruction {
            0 => Some(OpCode::BR),
            1 => Some(OpCode::ADD),
            2 => Some(OpCode::LD),
            3 => Some(OpCode::ST),
            4 => Some(OpCode::JSR),
            5 => Some(OpCode::AND),
            6 => Some(OpCode::LDR),
            7 => Some(OpCode::STR),
            8 => Some(OpCode::RTI),
            9 => Some(OpCode::NOT),
            10 => Some(OpCode::LDI),
            11 => Some(OpCode::STI),
            12 => Some(OpCode::JMP),
            13 => Some(OpCode::RES),
            14 => Some(OpCode::LEA),
            15 => Some(OpCode::TRAP),
            _ => None,
        }
    }
}

pub enum TrapCode
{
    GETC = 0x20,
    OUT = 0x21,
    PUTS = 0x22,
    IN = 0x23,
    PUTSP = 0x24,
    HALT = 0x25,
}

impl TrapCode
{
    pub fn handle_trap(trap: TrapCode, vm: &mut VM)
    {
        match trap
        {
            TrapCode::GETC => {
                let mut buffer = [0; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                vm.reg.general[0] = buffer[0] as u16;
            }
            TrapCode::OUT => {
                let c = vm.reg.general[0];
                print!("{}", c as u8 as char);
            }
            TrapCode::PUTS => {
                let mut pc = vm.reg.pc;
                let mem = vm.memory.clone();
                loop {
                    let c = mem[pc as usize];
                    if c == 0
                    {
                        break;
                    }
                    print!("{}", c as u8 as char);
                    pc += 1;
                }
            }
            TrapCode::IN => {
               let input = Self::trap_in();
            }
            TrapCode::PUTSP => {
                let mut pc = vm.reg.pc;
                loop {
                    let mem_val = vm.memory.get(pc as usize).copied();
                    match mem_val 
                    {
                        Some(val) => {
                            let c1 = (val & 0x00FF) as u8 as char;
                            if c1 == '\0'
                            {
                                break;
                            }
                            print!("{}", c1);
                            let c2 = !((val & 0xFF00) >> 8) as u8 as char;
                            if c2 == '\0'
                            {
                                break;
                            }
                            print!("{}", c2);
                            pc += 1;
                        }
                        None => break,
                    }
                }
            }
            TrapCode::HALT => {
                process::exit(0);
            }
        }
    }

    fn trap_in() -> u16 
    {
        print!("Enter a character: ");
        std::io::stdout().flush().unwrap();
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        buffer[0] as u16
    }
}


