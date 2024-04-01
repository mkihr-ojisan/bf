use crate::instruction::Instruction;

pub fn optimize(instructions: &[Instruction], optimize_level: i64) -> Vec<Instruction> {
    match optimize_level {
        0 => instructions.to_vec(),
        1 => pattern_match(instructions),
        2 => calculate_loop(&pattern_match(instructions)),
        _ => panic!("Invalid optimize level: {}", optimize_level),
    }
}

fn pattern_match(instructions: &[Instruction]) -> Vec<Instruction> {
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
                let optimized_loop = pattern_match(loop_instructions);
                optimized.push(Instruction::Loop(optimized_loop));
            }
            inst => optimized.push(inst.clone()),
        }

        i += 1;
    }

    optimized
}

const MEMORY_SIZE: usize = 1024;
const MEMORY_OFFSET: usize = MEMORY_SIZE / 2;

fn calculate_loop(instructions: &[Instruction]) -> Vec<Instruction> {
    let (optimized, _) = do_calculate_loop(instructions);
    optimized
}

fn do_calculate_loop(instructions: &[Instruction]) -> (Vec<Instruction>, bool) {
    let mut optimized = Vec::new();
    let mut can_be_calculated = true;
    for inst in instructions {
        match inst {
            Instruction::PutChar | Instruction::GetChar => {
                can_be_calculated = false;
                optimized.push(inst.clone());
            }
            Instruction::Loop(loop_instructions) => {
                can_be_calculated = false;

                let (loop_instructions, loop_can_be_calculated) =
                    do_calculate_loop(loop_instructions);

                if loop_can_be_calculated {
                    if let Some(simulation_result) = simulate(&loop_instructions) {
                        if simulation_result.pointer_offset == 0 {
                            if simulation_result.memory[MEMORY_OFFSET] == 1
                                || simulation_result.memory[MEMORY_OFFSET] == 255
                            {
                                let mut optimized_loop = Vec::new();

                                if simulation_result.memory[MEMORY_OFFSET] == 1 {
                                    optimized_loop.push(Instruction::Negate);
                                }

                                let mut offset = 0isize;
                                for inst in loop_instructions {
                                    match inst {
                                        Instruction::Increment => {
                                            if offset != 0 {
                                                optimized_loop
                                                    .push(Instruction::AddValueAt(-offset));
                                            }
                                        }
                                        Instruction::Decrement => {
                                            if offset != 0 {
                                                optimized_loop
                                                    .push(Instruction::SubtractValueAt(-offset));
                                            }
                                        }
                                        Instruction::PointerIncrement => {
                                            offset += 1;
                                            optimized_loop.push(Instruction::PointerIncrement);
                                        }
                                        Instruction::PointerDecrement => {
                                            offset -= 1;
                                            optimized_loop.push(Instruction::PointerDecrement);
                                        }
                                        Instruction::Add(value) => {
                                            optimized_loop.push(Instruction::AddValueMultipliedBy(
                                                value, -offset,
                                            ));
                                        }
                                        Instruction::Subtract(value) => {
                                            optimized_loop.push(
                                                Instruction::SubtractValueMultipliedBy(
                                                    value, -offset,
                                                ),
                                            );
                                        }
                                        Instruction::SetZero => {
                                            optimized_loop.push(Instruction::SetZero);
                                        }
                                        Instruction::PointerAdd(value) => {
                                            offset += value as isize;
                                            optimized_loop.push(Instruction::PointerAdd(value));
                                        }
                                        Instruction::PointerSubtract(value) => {
                                            offset -= value as isize;
                                            optimized_loop
                                                .push(Instruction::PointerSubtract(value));
                                        }

                                        Instruction::PutChar
                                        | Instruction::GetChar
                                        | Instruction::Loop(_)
                                        | Instruction::AddValueAt(_)
                                        | Instruction::SubtractValueAt(_)
                                        | Instruction::AddValueMultipliedBy(_, _)
                                        | Instruction::SubtractValueMultipliedBy(_, _)
                                        | Instruction::Negate
                                        | Instruction::IfNotZero(_) => {
                                            unreachable!()
                                        }
                                    }
                                }
                                optimized_loop.push(Instruction::SetZero);

                                if optimized_loop.is_empty() {
                                    optimized.push(Instruction::SetZero);
                                } else {
                                    optimized_loop.push(Instruction::SetZero);
                                    optimized.push(Instruction::IfNotZero(optimized_loop));
                                }
                            } else {
                                optimized.push(Instruction::Loop(loop_instructions));
                            }
                        } else {
                            optimized.push(Instruction::Loop(loop_instructions));
                        }
                    } else {
                        optimized.push(Instruction::Loop(loop_instructions));
                    }
                } else {
                    optimized.push(Instruction::Loop(loop_instructions));
                }
            }
            _ => {
                optimized.push(inst.clone());
            }
        }
    }
    (optimized, can_be_calculated)
}

struct SimulationResult {
    memory: Vec<u8>,
    pointer_offset: isize,
}

fn simulate(instructions: &[Instruction]) -> Option<SimulationResult> {
    let mut memory = vec![0u8; MEMORY_SIZE];
    let mut pointer = MEMORY_OFFSET;
    let mut i = 0;
    while i < instructions.len() {
        match instructions[i] {
            Instruction::Increment => {
                memory[pointer] = memory[pointer].wrapping_add(1);
            }
            Instruction::Decrement => {
                memory[pointer] = memory[pointer].wrapping_sub(1);
            }
            Instruction::PointerIncrement => {
                if pointer == MEMORY_SIZE - 1 {
                    return None;
                }
                pointer += 1;
            }
            Instruction::PointerDecrement => {
                if pointer == 0 {
                    return None;
                }
                pointer -= 1;
            }
            Instruction::Loop(_) => {
                return None;
            }
            Instruction::GetChar | Instruction::PutChar => unreachable!(),
            Instruction::Add(value) => {
                memory[pointer] = memory[pointer].wrapping_add(value);
            }
            Instruction::Subtract(value) => {
                memory[pointer] = memory[pointer].wrapping_sub(value);
            }
            Instruction::SetZero => {
                return None;
            }
            Instruction::PointerAdd(value) => {
                if pointer + value >= MEMORY_SIZE {
                    return None;
                }
                pointer += value;
            }
            Instruction::PointerSubtract(value) => {
                if pointer < value {
                    return None;
                }
                pointer -= value;
            }
            Instruction::AddValueAt(_)
            | Instruction::SubtractValueAt(_)
            | Instruction::AddValueMultipliedBy(_, _)
            | Instruction::SubtractValueMultipliedBy(_, _)
            | Instruction::Negate
            | Instruction::IfNotZero(_) => {
                return None;
            }
        }

        i += 1;
    }

    Some(SimulationResult {
        memory,
        pointer_offset: pointer as isize - MEMORY_OFFSET as isize,
    })
}
