use lc3_vm::register::Registers;

const MEMORY_SIZE: usize = 1 << 16;
pub struct VM 
{
    memory: [u16; MEMORY_SIZE],
    reg: Registers,
}
