#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Opcode {
    Halt,
    Add,
    Sub,
    Mul,
    Div,
    Swap1,
    Swap2,
    Swap3,
    Swap4,
    Swap5,
    Swap6,
    Swap7,
}

impl Opcode {
    pub fn to_u8(&self) -> u8 {
        match self {
            Opcode::Halt => 0x00,
            Opcode::Add => 0x01,
            Opcode::Sub => 0x02,
            Opcode::Mul => 0x03,
            Opcode::Div => 0x04,
            Opcode::Swap1 => 0x05,
            Opcode::Swap2 => 0x06,
            Opcode::Swap3 => 0x07,
            Opcode::Swap4 => 0x08,
            Opcode::Swap5 => 0x09,
            Opcode::Swap6 => 0x0a,
            Opcode::Swap7 => 0x0b,
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Opcode::Halt),
            0x01 => Ok(Opcode::Add),
            0x02 => Ok(Opcode::Sub),
            0x03 => Ok(Opcode::Mul),
            0x04 => Ok(Opcode::Div),
            0x05 => Ok(Opcode::Swap1),
            0x06 => Ok(Opcode::Swap2),
            0x07 => Ok(Opcode::Swap3),
            0x08 => Ok(Opcode::Swap4),
            0x09 => Ok(Opcode::Swap5),
            0x0a => Ok(Opcode::Swap6),
            0x0b => Ok(Opcode::Swap7),
            _ => Err(()),
        }
    }
}
