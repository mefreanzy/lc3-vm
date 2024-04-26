use std::io::{Read, Write};
use std::process;
use crate::{lc3::VM, register::ConditionFlag};

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
    pub fn execute(&self, vm: &mut VM, instruction: u16)
    {
        match self {
            OpCode::BR => {
                let offset = Self::sign_extend(instruction & 0x1FF, 9);
                if vm.reg.cond.contains(ConditionFlag::ZRO) 
                {
                    let cond_flag = ConditionFlag::from_bits(offset as u16);
                    vm.reg.cond = vm.reg.cond | cond_flag.expect("Expecting a non-empty value for the conditional execution of the BR' opcode");
                }
                }
            OpCode::ADD => {
                let dr = (instruction >> 9) & 0x7;
                let sr1 = (instruction >> 6) & 0x7;
                let imm_flag = (instruction >> 5) & 0x1;
                if imm_flag != 0
                {
                    let imm5 = Self::sign_extend(instruction & 0x1F, 5);
                    let sr1_val = vm.reg.general[sr1 as usize];
                    vm.reg.general[dr as usize] = sr1_val.wrapping_add(imm5);
                } else {
                    let sr2 = instruction & 0x7;
                    let sr1_val = vm.reg.general[sr1 as usize];
                    let sr2_val = vm.reg.general[sr2 as usize];
                    vm.reg.general[dr as usize] = sr1_val.wrapping_add(sr2_val);
                }
                vm.reg.update_flags(vm.reg.general[dr as usize]);
            }
            OpCode::LD => {
                let dr = (instruction >> 9) & 0x7;
                let pc_offset = Self::sign_extend(instruction & 0x1FF, 9);
                let addr = vm.reg.pc.wrapping_add(pc_offset);
                vm.reg.general[dr as usize] = vm.memory[addr as usize];
                vm.reg.update_flags(vm.reg.general[dr as usize]);
            }
            OpCode::ST => {
                let sr = (instruction >> 9) & 0x7;
                let pc_offset = Self::sign_extend(instruction & 0x1FF, 9);
                let addr = vm.reg.pc.wrapping_add(pc_offset);
                vm.memory[addr as usize] = vm.reg.general[sr as usize];
            }
            OpCode::JSR => {
                let long_flag = (instruction >> 11) & 0x1;
                let pc_offset = if long_flag != 0 {
                    Self::sign_extend(instruction & 0x7FF, 11)
                } else {
                    Self::sign_extend(instruction & 0x1FF, 9)
                };
                let r7 = vm.reg.general[7];
                vm.reg.general[7] = vm.reg.pc;
                if long_flag != 0 
                {
                    let r = (instruction >> 6) & 0x7;
                    let val = vm.reg.general[r as usize];
                    vm.reg.pc = val;
                } else {
                    vm.reg.pc = vm.reg.pc.wrapping_add(pc_offset);
                }
                vm.reg.pc = r7;
            }
        }
    }

    fn sign_extend(x: u16, bit: usize) -> u16
    {
        if (x >> (bit - 1)) & 1 != 0
        {
            x | (0xFFFF << bit)
        } else {
            x
        }
    }

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


