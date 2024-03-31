use crate::{instruction::Instruction, vm::VMInstruction};

pub fn compile(instructions: &[Instruction]) -> Vec<VMInstruction> {
    do_compile(instructions, 0)
}

fn do_compile(instructions: &[Instruction], offset: usize) -> Vec<VMInstruction> {
    let mut compiled = Vec::new();
    for inst in instructions {
        match inst {
            Instruction::Increment => compiled.push(VMInstruction::Increment),
            Instruction::Decrement => compiled.push(VMInstruction::Decrement),
            Instruction::PointerIncrement => compiled.push(VMInstruction::PointerIncrement),
            Instruction::PointerDecrement => compiled.push(VMInstruction::PointerDecrement),
            Instruction::PutChar => compiled.push(VMInstruction::PutChar),
            Instruction::GetChar => compiled.push(VMInstruction::GetChar),
            Instruction::Loop(loop_instructions) => {
                let start = compiled.len();
                compiled.push(VMInstruction::JumpIfZero(0));
                compiled.extend(do_compile(loop_instructions, start + offset + 1));
                let end = compiled.len();
                compiled.push(VMInstruction::JumpIfNotZero(start + offset));
                compiled[start] = VMInstruction::JumpIfZero(end + offset);
            }
            Instruction::Add(value) => compiled.push(VMInstruction::Add(*value)),
            Instruction::Subtract(value) => compiled.push(VMInstruction::Subtract(*value)),
            Instruction::SetZero => compiled.push(VMInstruction::SetZero),
            Instruction::PointerAdd(value) => compiled.push(VMInstruction::PointerAdd(*value)),
            Instruction::PointerSubtract(value) => {
                compiled.push(VMInstruction::PointerSubtract(*value))
            }
            Instruction::AddValueAt(value) => compiled.push(VMInstruction::AddValueAt(*value)),
            Instruction::SubtractValueAt(value) => {
                compiled.push(VMInstruction::SubtractValueAt(*value))
            }
            Instruction::AddValueMultipliedBy(multiplier, offset) => {
                compiled.push(VMInstruction::AddValueMultipliedBy(*multiplier, *offset))
            }
            Instruction::SubtractValueMultipliedBy(multiplier, offset) => compiled.push(
                VMInstruction::SubtractValueMultipliedBy(*multiplier, *offset),
            ),
            Instruction::Negate => compiled.push(VMInstruction::Negate),
            Instruction::IfNotZero(if_instructions) => {
                let start = compiled.len();
                compiled.push(VMInstruction::JumpIfZero(0));
                compiled.extend(do_compile(if_instructions, start + offset + 1));
                let end = compiled.len() - 1;
                compiled[start] = VMInstruction::JumpIfZero(end + offset);
            }
        }
    }

    compiled
}
