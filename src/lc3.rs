use crate::register::Registers;

const MEMORY_SIZE: usize = 1 << 16;
pub struct VM 
{
    pub memory: [u16; MEMORY_SIZE],
    pub reg: Registers,
}
