use std::str::Chars;

use anyhow::bail;

use crate::instruction::Instruction;

pub fn parse(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let mut chars = input.chars();
    let program = do_parse(&mut chars);
    if chars.next().is_some() {
        bail!("Unexpected end of loop");
    }
    Ok(program)
}

fn do_parse(chars: &mut Chars) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            '+' => instructions.push(Instruction::Increment),
            '-' => instructions.push(Instruction::Decrement),
            '>' => instructions.push(Instruction::PointerIncrement),
            '<' => instructions.push(Instruction::PointerDecrement),
            '.' => instructions.push(Instruction::PutChar),
            ',' => instructions.push(Instruction::GetChar),
            '[' => instructions.push(Instruction::Loop(do_parse(chars))),
            ']' => return instructions,
            _ => {}
        }
    }
    instructions
}
