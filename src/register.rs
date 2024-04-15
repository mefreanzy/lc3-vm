pub struct Registers
{
    pub general: Vec<u16>,
    pub pc: u16,
    pub cond: u16,
}

impl Registers
{
    pub fn new(reg_count: usize) -> Registers
    {
        Registers {
            general: vec![0; reg_count],
            pc: 0,
            cond: 0,
        }
    }

    pub fn get_reg(&self, index: usize) -> Option<u16>
    {
        self.general.get(index).copied()
    }

    pub fn set_reg(&mut self, index: usize, value: u16) -> bool
    {
        if let Some(reg) = self.general.get_mut(index)
        {
            *reg = value
        } else {
            false
        }
    }

    pub fn update_flags(&mut self, result: u16)
    {
        if result == 0
        {
            self.cond = 1 << 1; 
        } else if (result >> 15) == 1 {
            self.cond = 1 << 0;
        } else {
            self.cond = 1 << 2;
        }
    }
}
