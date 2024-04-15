pub enum ConditionFlag
{
    ZRO = 1 << 1,
    POS = 1 << 0,
    NEG = 1 << 2,
}

pub struct Registers
{
    pub general: Vec<u16>,
    pub pc: u16,
    pub cond: ConditionFlag,
}

impl Registers
{
    pub fn new(reg_count: usize) -> Registers
    {
        Registers {
            general: vec![0; reg_count],
            pc: 0,
            cond: ConditionFlag::ZRO,
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
            *reg = value;
            true
        } else {
            false
        }
    }

    pub fn update_flags(&mut self, result: u16)
    {
        if result == 0
        {
            self.cond = ConditionFlag::ZRO;
        } else if (result >> 15) == 1 {
            self.cond = ConditionFlag::POS;
        } else {
            self.cond = ConditionFlag::NEG;
        }
    }
}
