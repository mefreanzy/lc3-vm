use bitflags::bitflags;

bitflags!
{
    pub struct ConditionFlag: u16
    {
        const ZRO = 0b0000000000000010;
        const POS = 0b0000000000000001;
        const NEG = 0b0000000000000100;
    }
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

    pub fn update_flags(&mut self, dr: u16)
    {
        let val = self.general[dr as usize];
        self.cond = if val == 0 {
            ConditionFlag::ZRO
        } else if (val & 0x8000) != 0 {
            ConditionFlag::NEG
        } else {
            ConditionFlag::POS
        };
    }
}
