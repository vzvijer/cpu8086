use std::fmt::Display;


#[derive(Debug)]
pub enum ModeProp {
    Unknown = -1,
    MemoryNoDisplacement,
    Memory8BitDisplacement,
    Memory16BitDisplacement,
    Register,
}

impl From<u8> for ModeProp {
    fn from(value: u8) -> Self {
        match value {
            0 => ModeProp::MemoryNoDisplacement,
            1 => ModeProp::Memory8BitDisplacement,
            2 => ModeProp::Memory16BitDisplacement,
            3 => ModeProp::Register,
            _ => ModeProp::Unknown,
        }
    }
}


#[derive(Debug)]
pub enum RegisterProp {
    Unknown = -1,
    AL,
    AX,
    CL,
    CX,
    DL,
    DX,
    BL,
    BX,
    AH,
    SP,
    CH,
    BP,
    DH,
    SI,
    BH,
    DI,
}

impl From<u8> for RegisterProp {
    fn from(value: u8) -> Self {
        match value {
            0 => RegisterProp::AL,
            1 => RegisterProp::AX,
            2 => RegisterProp::CL,
            3 => RegisterProp::CX,
            4 => RegisterProp::DL,
            5 => RegisterProp::DX,
            6 => RegisterProp::BL,
            7 => RegisterProp::BX,
            8 => RegisterProp::AH,
            9 => RegisterProp::SP,
            10 => RegisterProp::CH,
            11 => RegisterProp::BP,
            12 => RegisterProp::DH,
            13 => RegisterProp::SI,
            14 => RegisterProp::BH,
            15 => RegisterProp::DI,
            _ => RegisterProp::Unknown,
        }
    }
}

pub enum SegmentRegProp {
    Unknown = -1,
    ES,
    CS,
    SS,
    DS,
}


#[derive(Debug)]
pub struct MovProps {
    pub word: bool,
    pub mode: ModeProp,
    pub register: RegisterProp,
    pub reg_mem: u8,
    pub displacement: u16,
    pub data: u16,
    pub address: u16,
    pub segment_reg: u8,
}

impl Default for MovProps {
    fn default() -> Self {
        MovProps {
            word: false,
            mode: ModeProp::Unknown,
            register: RegisterProp::Unknown,
            reg_mem: 0,
            displacement: 0,
            data: 0,
            address: 0,
            segment_reg: 0,
        }
    }
}


#[derive(Debug)]
pub enum InstrName {
    Unknown = -1,
    MOV,
}

#[derive(Debug)]
pub struct Instruction {
    pub name: InstrName,
    pub props: MovProps,
    pub code: Vec<u8>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.name {
            InstrName::MOV => {
                f.write_str("MOV");
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
