use crate::instruction::Instruction;

use self::{consecutive_inc_dec::optimize_consecutive_inc_dec, mul_loop::optimize_mul_loop};

mod consecutive_inc_dec;
mod mul_loop;

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
#[clap(rename_all = "snake_case")]
pub enum Optimization {
    All,

    ConsecutiveIncDec,
    MulLoop,
}

pub fn optimize(mut instructions: Vec<Instruction>, options: &[Optimization]) -> Vec<Instruction> {
    let all = options.contains(&Optimization::All);

    if all || options.contains(&Optimization::ConsecutiveIncDec) {
        instructions = optimize_consecutive_inc_dec(&instructions);
    }
    if all || options.contains(&Optimization::MulLoop) {
        instructions = optimize_mul_loop(&instructions);
    }
    instructions
}
