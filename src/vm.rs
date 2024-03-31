use std::io::{ErrorKind, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMInstruction {
    Increment,
    Decrement,
    PointerIncrement,
    PointerDecrement,
    PutChar,
    GetChar,

    JumpIfZero(usize),
    JumpIfNotZero(usize),

    Add(u8),
    Subtract(u8),
    SetZero,
    PointerAdd(usize),
    PointerSubtract(usize),
    AddValueAt(isize),
    SubtractValueAt(isize),
    AddValueMultipliedBy(u8, isize),
    SubtractValueMultipliedBy(u8, isize),
    Negate,
}

pub fn run(instructions: &[VMInstruction], trace: bool) {
    let mut memory = vec![0u8; 30000];
    let mut pointer = 0;
    let mut instruction_pointer = 0;
    while instruction_pointer < instructions.len() {
        if trace {
            println!(
                "ip: {}, inst: {:?}, ptr: {}, mem: {}\n{:?}",
                instruction_pointer,
                instructions[instruction_pointer],
                pointer,
                memory[pointer],
                &memory[0..30]
            );
        }
        match instructions[instruction_pointer] {
            VMInstruction::Increment => {
                memory[pointer] = memory[pointer].wrapping_add(1);
            }
            VMInstruction::Decrement => {
                memory[pointer] = memory[pointer].wrapping_sub(1);
            }
            VMInstruction::PointerIncrement => {
                pointer += 1;
            }
            VMInstruction::PointerDecrement => {
                pointer -= 1;
            }
            VMInstruction::PutChar => {
                print!("{}", memory[pointer] as char);
                std::io::stdout().flush().unwrap();
            }
            VMInstruction::GetChar => {
                let mut input = [0];
                match std::io::stdin().read_exact(&mut input) {
                    Ok(()) => {
                        memory[pointer] = input[0];
                    }
                    Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                        memory[pointer] = 0;
                    }
                    Err(e) => {
                        panic!("Error reading input: {:?}", e);
                    }
                }
            }
            VMInstruction::JumpIfZero(jump) => {
                if memory[pointer] == 0 {
                    instruction_pointer = jump;
                }
            }
            VMInstruction::JumpIfNotZero(jump) => {
                if memory[pointer] != 0 {
                    instruction_pointer = jump;
                }
            }
            VMInstruction::Add(value) => {
                memory[pointer] = memory[pointer].wrapping_add(value);
            }
            VMInstruction::Subtract(value) => {
                memory[pointer] = memory[pointer].wrapping_sub(value);
            }
            VMInstruction::SetZero => {
                memory[pointer] = 0;
            }
            VMInstruction::PointerAdd(value) => {
                pointer += value;
            }
            VMInstruction::PointerSubtract(value) => {
                pointer -= value;
            }
            VMInstruction::AddValueAt(at) => {
                memory[pointer] =
                    memory[pointer].wrapping_add(memory[(pointer as isize + at) as usize]);
            }
            VMInstruction::SubtractValueAt(at) => {
                memory[pointer] =
                    memory[pointer].wrapping_sub(memory[(pointer as isize + at) as usize]);
            }
            VMInstruction::AddValueMultipliedBy(value, at) => {
                memory[pointer] = memory[pointer]
                    .wrapping_add(value.wrapping_mul(memory[(pointer as isize + at) as usize]));
            }
            VMInstruction::SubtractValueMultipliedBy(value, at) => {
                memory[pointer] = memory[pointer]
                    .wrapping_sub(value.wrapping_mul(memory[(pointer as isize + at) as usize]));
            }
            VMInstruction::Negate => {
                memory[pointer] = memory[pointer].wrapping_neg();
            }
        }
        instruction_pointer += 1;
    }
}
