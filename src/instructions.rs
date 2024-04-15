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

