use crate::{
    assembler::x86_64::{addressing_mode::AddressingMode, Assembler, ByteRegister, QwordRegister},
    instruction::Instruction,
};

extern "C" {
    fn putchar(c: i32);
    fn getchar() -> i32;
}

const POINTER_REGISTER: QwordRegister = QwordRegister::Rdx; // arg1

pub fn compile(instructions: &[Instruction]) -> Vec<u8> {
    let mut assembler = Assembler::new();

    assembler.push_r64(QwordRegister::Rbp);
    assembler.mov_rm64_r64(
        AddressingMode::Register {
            reg: QwordRegister::Rbp,
        },
        QwordRegister::Rsp,
    );

    // メモリのアドレスは第1引数に渡される
    assembler.mov_rm64_r64(
        AddressingMode::Register {
            reg: POINTER_REGISTER,
        },
        QwordRegister::Rdi,
    );

    do_compile(instructions, &mut assembler);

    assembler.pop_r64(QwordRegister::Rbp);
    assembler.ret();

    assembler.code
}

fn do_compile(instructions: &[Instruction], assembler: &mut Assembler) {
    for inst in instructions {
        match inst {
            Instruction::Increment => {
                assembler.inc_rm8(AddressingMode::Indirect {
                    reg: POINTER_REGISTER,
                });
            }
            Instruction::Decrement => {
                assembler.dec_rm8(AddressingMode::Indirect {
                    reg: POINTER_REGISTER,
                });
            }
            Instruction::PointerIncrement => {
                assembler.add_rm64_imm8(
                    AddressingMode::Register {
                        reg: POINTER_REGISTER,
                    },
                    1,
                );
            }
            Instruction::PointerDecrement => {
                assembler.sub_rm64_imm8(
                    AddressingMode::Register {
                        reg: POINTER_REGISTER,
                    },
                    1,
                );
            }
            Instruction::PutChar => {
                assembler.movzx_r32_rm8(
                    QwordRegister::Rdi,
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                );

                assembler.mov_r64_imm64(QwordRegister::Rax, putchar as usize as u64);

                assembler.push_r64(POINTER_REGISTER);
                assembler.call_rm64(AddressingMode::Register {
                    reg: QwordRegister::Rax,
                });
                assembler.pop_r64(POINTER_REGISTER);
            }
            Instruction::GetChar => {
                assembler.mov_r64_imm64(QwordRegister::Rax, getchar as usize as u64);

                assembler.push_r64(POINTER_REGISTER);
                assembler.call_rm64(AddressingMode::Register {
                    reg: QwordRegister::Rax,
                });
                assembler.pop_r64(POINTER_REGISTER);

                assembler.mov_rm8_r8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    ByteRegister::Al,
                );
            }
            Instruction::Loop(loop_instructions) => {
                assembler.cmp_rm8_imm8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    0,
                );

                let jump = assembler.code.len();
                assembler.je_rel32(0);

                let start = assembler.code.len();
                do_compile(loop_instructions, assembler);

                assembler.cmp_rm8_imm8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    0,
                );
                assembler.jne_rel32((start as isize - assembler.code.len() as isize - 6) as i32);

                let end = assembler.code.len();

                assembler.set_je_rel32(jump, (end - jump - 6) as i32);
            }
            Instruction::Add(value) => {
                assembler.add_rm8_imm8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    *value,
                );
            }
            Instruction::Subtract(value) => {
                assembler.sub_rm8_imm8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    *value,
                );
            }
            Instruction::SetZero => {
                assembler.mov_rm8_imm8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    0,
                );
            }
            Instruction::PointerAdd(value) => {
                if *value >= u8::MAX as usize {
                    panic!("value is too large");
                }

                assembler.add_rm64_imm8(
                    AddressingMode::Register {
                        reg: POINTER_REGISTER,
                    },
                    *value as u8,
                );
            }
            Instruction::PointerSubtract(value) => {
                if *value >= u8::MAX as usize {
                    panic!("value is too large");
                }

                assembler.sub_rm64_imm8(
                    AddressingMode::Register {
                        reg: POINTER_REGISTER,
                    },
                    *value as u8,
                );
            }
            Instruction::AddValueAt(at) => {
                assembler.mov_r8_rm8(
                    ByteRegister::Al,
                    AddressingMode::IndirectDisplacement32 {
                        base: POINTER_REGISTER,
                        disp: *at as i32,
                    },
                );
                assembler.add_rm8_r8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    ByteRegister::Al,
                );
            }
            Instruction::SubtractValueAt(at) => {
                assembler.mov_r8_rm8(
                    ByteRegister::Al,
                    AddressingMode::IndirectDisplacement32 {
                        base: POINTER_REGISTER,
                        disp: *at as i32,
                    },
                );
                assembler.sub_rm8_r8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    ByteRegister::Al,
                );
            }
            Instruction::AddValueMultipliedBy(mul, at) => {
                assembler.mov_r8_rm8(
                    ByteRegister::Al,
                    AddressingMode::IndirectDisplacement32 {
                        base: POINTER_REGISTER,
                        disp: *at as i32,
                    },
                );

                assembler.mov_rm8_imm8(
                    AddressingMode::Register {
                        reg: QwordRegister::Rcx,
                    },
                    *mul,
                );

                assembler.mul_rm8(AddressingMode::Register {
                    reg: QwordRegister::Rcx,
                });

                assembler.add_rm8_r8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    ByteRegister::Al,
                );
            }
            Instruction::SubtractValueMultipliedBy(mul, at) => {
                assembler.mov_r8_rm8(
                    ByteRegister::Al,
                    AddressingMode::IndirectDisplacement32 {
                        base: POINTER_REGISTER,
                        disp: *at as i32,
                    },
                );

                assembler.mov_rm8_imm8(
                    AddressingMode::Register {
                        reg: QwordRegister::Rcx,
                    },
                    *mul,
                );

                assembler.mul_rm8(AddressingMode::Register {
                    reg: QwordRegister::Rcx,
                });

                assembler.sub_rm8_r8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    ByteRegister::Al,
                );
            }
            Instruction::Negate => {
                assembler.neg_rm8(AddressingMode::Indirect {
                    reg: POINTER_REGISTER,
                });
            }
            Instruction::IfNotZero(if_instructions) => {
                assembler.cmp_rm8_imm8(
                    AddressingMode::Indirect {
                        reg: POINTER_REGISTER,
                    },
                    0,
                );

                let jump = assembler.code.len();
                assembler.je_rel32(0);

                do_compile(if_instructions, assembler);

                let end = assembler.code.len();

                assembler.set_je_rel32(jump, (end - jump - 6) as i32);
            }
        }
    }
}
