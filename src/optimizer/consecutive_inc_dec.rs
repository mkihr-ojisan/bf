use crate::instruction::Instruction;

pub fn optimize_consecutive_inc_dec(instructions: &[Instruction]) -> Vec<Instruction> {
    let mut i = 0;
    let mut optimized = Vec::new();
    while i < instructions.len() {
        let inst = &instructions[i];
        match inst {
            // 連続するIncrementをAddに変換
            Instruction::Increment if instructions.get(i + 1) == Some(&Instruction::Increment) => {
                let mut count = 1;
                while instructions.get(i + 1) == Some(&Instruction::Increment) {
                    count += 1;
                    i += 1;
                }
                optimized.push(Instruction::Add(count));
            }
            // 連続するDecrementをSubtractに変換
            Instruction::Decrement if instructions.get(i + 1) == Some(&Instruction::Decrement) => {
                let mut count = 1;
                while instructions.get(i + 1) == Some(&Instruction::Decrement) {
                    count += 1;
                    i += 1;
                }
                optimized.push(Instruction::Subtract(count));
            }
            // 連続するPointerIncrementをPointerAddに変換
            Instruction::PointerIncrement
                if instructions.get(i + 1) == Some(&Instruction::PointerIncrement) =>
            {
                let mut count = 1;
                while instructions.get(i + 1) == Some(&Instruction::PointerIncrement) {
                    count += 1;
                    i += 1;
                }
                optimized.push(Instruction::PointerAdd(count));
            }
            // 連続するPointerDecrementをPointerSubtractに変換
            Instruction::PointerDecrement
                if instructions.get(i + 1) == Some(&Instruction::PointerDecrement) =>
            {
                let mut count = 1;
                while instructions.get(i + 1) == Some(&Instruction::PointerDecrement) {
                    count += 1;
                    i += 1;
                }
                optimized.push(Instruction::PointerSubtract(count));
            }
            // ループの中身を最適化
            Instruction::Loop(loop_instructions) => {
                let optimized_loop = optimize_consecutive_inc_dec(loop_instructions);
                optimized.push(Instruction::Loop(optimized_loop));
            }
            inst => optimized.push(inst.clone()),
        }

        i += 1;
    }

    optimized
}
