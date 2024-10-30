use std::fs::File;
use std::io::{self, Read};
use std::{env, process};

use instruction::{InstrName, Instruction, ModeProp, MovProps, RegisterProp};

mod instruction;

fn decode_instruction(bytes: Vec<u8>) -> Result<(Instruction, u8), ()> {
    if ((bytes[0] & 0b1111_1100) ^ 0b1000_1000) == 0 {
        Ok((
            Instruction {
                name: InstrName::MOV,
                props: MovProps {
                    word: (bytes[0] & 0b0000_0001) > 0,
                    mode: ModeProp::from(bytes[1] & 0b1110_0000 >> 5),
                    register: RegisterProp::from(bytes[1] & 0b0001_1100 >> 2),
                    reg_mem: bytes[1] & 0b0000_0011,
                    displacement: u16::from(bytes[3]) << 8 & u16::from(bytes[2]),
                    ..MovProps::default()
                },
                code: bytes,
            },
            4,
        ))
    } else if ((bytes[0] & 0b1111_1110) ^ 0b1100_0110) == 0 {
        Ok((
            Instruction {
                name: InstrName::MOV,
                props: MovProps {
                    word: (bytes[0] & 0b0000_0001) > 0,
                    mode: ModeProp::from(bytes[1] & 0b1110_0000 >> 5),
                    register: RegisterProp::from(bytes[1] & 0b0001_1100 >> 2),
                    reg_mem: bytes[1] & 0b0000_0011,
                    displacement: u16::from(bytes[3]) << 8 & u16::from(bytes[2]),
                    data: u16::from(bytes[5]) << 8 & u16::from(bytes[4]),
                    ..MovProps::default()
                },
                code: bytes,
            },
            6,
        ))
    } else if ((bytes[0] & 0b1111_0000) ^ 0b1011_0000) == 0 {
        Ok((
            Instruction {
                name: InstrName::MOV,
                props: MovProps {
                    word: (bytes[0] & 0b0000_1000) > 0,
                    register: RegisterProp::from(bytes[1] & 0b0001_1100 >> 2),
                    data: u16::from(bytes[2]) << 8 & u16::from(bytes[1]),
                    ..MovProps::default()
                },
                code: bytes,
            },
            3,
        ))
    } else if ((bytes[0] & 0b1111_1110) ^ 0b1010_0000) == 0 {
        Ok((
            Instruction {
                name: InstrName::MOV,
                props: MovProps {
                    word: (bytes[0] & 0b0000_0001) > 0,
                    address: u16::from(bytes[2]) << 8 & u16::from(bytes[1]),
                    ..MovProps::default()
                },
                code: bytes,
            },
            3,
        ))
    } else {
        Ok((
            Instruction {
                name: InstrName::Unknown,
                props: MovProps {
                    ..MovProps::default()
                },
                code: bytes,
            },
            1,
        ))
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let program_length = buffer.len();
    let mut i = 0;
    loop {
        let instr_bytes = (buffer[i..(if i + 6 >= program_length {
            program_length
        } else {
            i + 6
        })])
            .to_vec();

        match decode_instruction(instr_bytes) {
            Ok((instr, length)) => {
                println!("{:?}", instr);
                i += usize::from(length);
            }
            Err(()) => process::exit(1),
        };

        if i > program_length {
            process::exit(1);
        } else if i == program_length {
            break;
        }
    }

    Ok(())
}
