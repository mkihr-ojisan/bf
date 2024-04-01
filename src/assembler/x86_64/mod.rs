use self::addressing_mode::AddressingMode;

pub mod addressing_mode;

pub struct Assembler {
    pub code: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteRegister {
    Al,
    Cl,
    Dl,
    Bl,
    Ah,
    Ch,
    Dh,
    Bh,
    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QwordRegister {
    Rax, // caller-saved, return value
    Rcx, // caller-saved, argument 4
    Rdx, // caller-saved, argument 3
    Rbx,
    Rsp, // stack pointer
    Rbp, // stack base pointer
    Rsi, // argument 2
    Rdi, // argument 1
    R8,  // argument 5
    R9,  // argument 6
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Assembler {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    fn rex(&mut self, w: bool, r: bool, x: bool, b: bool) {
        // w: オペランドが64bitになる
        // r: ModR/M regが拡張される
        // x: SIB indexが拡張される
        // b: ModR/M r/m、SIB base、opcode regのいずれかが拡張される

        let mut rex = 0b0100_0000;
        if w {
            rex |= 0b1000;
        }
        if r {
            rex |= 0b0100;
        }
        if x {
            rex |= 0b0010;
        }
        if b {
            rex |= 0b0001;
        }
        if rex != 0b0100_0000 {
            self.code.push(rex);
        }
    }

    fn opcode(&mut self, opcode: u8) {
        self.code.push(opcode);
    }

    pub fn push_r64(&mut self, r64: QwordRegister) {
        // Opcode: 50+rd
        // Instruction: PUSH r64
        // Op/En: O (opcode + rd(r))
        // Description: Push r64.

        let register = r64 as u8;
        self.rex(false, false, false, register & 0b1000 != 0);
        self.opcode(0x50 + (register & 0b111));
    }

    pub fn pop_r64(&mut self, r64: QwordRegister) {
        // Opcode: 58+rd
        // Instruction: POP r64
        // Op/En: O (opcode + rd(w))
        // Description: Pop r64.

        let register = r64 as u8;
        self.rex(false, false, false, register & 0b1000 != 0);
        self.opcode(0x58 + (register & 0b111));
    }

    pub fn ret(&mut self) {
        // Opcode: C3
        // Instruction: RET
        // Op/En: ZO
        // Description: Near return to the calling procedure.

        self.opcode(0xc3);
    }

    pub fn inc_rm8(&mut self, rm8: AddressingMode) {
        // Opcode: FE /0
        // Instruction: INC r/m8
        // Op/En: M (ModRM:r/m (r, w))
        // Description: Increment r/m byte by 1.

        self.rex(false, false, rm8.rex_x(), rm8.rex_b());
        self.opcode(0xfe);
        self.code.push(rm8.mod_r_m(0));
        self.code.extend(rm8.sib());
        self.code.extend(rm8.displacement8());
        self.code.extend(rm8.displacement32().iter().flatten());
    }

    pub fn dec_rm8(&mut self, rm8: AddressingMode) {
        // Opcode: FE /1
        // Instruction: DEC r/m8
        // Op/En: M (ModRM:r/m (r, w))
        // Description: Decrement r/m byte by 1.

        self.rex(false, false, rm8.rex_x(), rm8.rex_b());
        self.opcode(0xfe);
        self.code.push(rm8.mod_r_m(1));
        self.code.extend(rm8.sib());
        self.code.extend(rm8.displacement8());
        self.code.extend(rm8.displacement32().iter().flatten());
    }

    pub fn inc_rm64(&mut self, rm64: AddressingMode) {
        // Opcode: REX.W + FF /0
        // Instruction: INC r/m64
        // Op/En: M (ModRM:r/m (r, w))
        // Description: Increment r/m quadword by 1.

        self.rex(true, false, rm64.rex_x(), rm64.rex_b());
        self.opcode(0xff);
        self.code.push(rm64.mod_r_m(0));
        self.code.extend(rm64.sib());
        self.code.extend(rm64.displacement8());
        self.code.extend(rm64.displacement32().iter().flatten());
    }

    pub fn dec_rm64(&mut self, rm64: AddressingMode) {
        // Opcode: REX.W + FF /1
        // Instruction: DEC r/m64
        // Op/En: M (ModRM:r/m (r, w))
        // Description: Decrement r/m64 by 1.

        self.rex(true, false, rm64.rex_x(), rm64.rex_b());
        self.opcode(0xff);
        self.code.push(rm64.mod_r_m(1));
        self.code.extend(rm64.sib());
        self.code.extend(rm64.displacement8());
        self.code.extend(rm64.displacement32().iter().flatten());
    }

    pub fn call_rm64(&mut self, rm64: AddressingMode) {
        // Opcode: FF /2
        // Instruction: CALL r/m64
        // Op/En: M (ModRM:r/m (r))
        // Description: Call near, absolute indirect, address given in r/m64.

        self.rex(false, false, rm64.rex_x(), rm64.rex_b());
        self.opcode(0xff);
        self.code.push(rm64.mod_r_m(2));
        self.code.extend(rm64.sib());
        self.code.extend(rm64.displacement8());
        self.code.extend(rm64.displacement32().iter().flatten());
    }

    pub fn mov_rm64_r64(&mut self, dest: AddressingMode, src: QwordRegister) {
        // Opcode: REX.W + 89 /r
        // Instruction: MOV r/m64, r64
        // Op/En: MR (ModRM:r/m (w), ModRM:reg (r))
        // Description: Move r64 to r/m64.

        let src = src as u8; // -> ModRM:reg

        self.rex(true, src & 0b1000 != 0, dest.rex_x(), dest.rex_b());
        self.opcode(0x89);
        self.code.push(dest.mod_r_m(src));
        self.code.extend(dest.sib());
        self.code.extend(dest.displacement8());
        self.code.extend(dest.displacement32().iter().flatten());
    }

    pub fn mov_r64_imm64(&mut self, dest: QwordRegister, src: u64) {
        // Opcode: REX.W + B8 /0 + rd io
        // Instruction: MOV r64, imm64
        // Op/En: OI (opcode + rd(w), imm64)
        // Description: Move imm64 to r64.

        let dest = dest as u8; // -> ModRM:r/m

        self.rex(true, false, false, dest & 0b1000 != 0);
        self.opcode(0xb8 + (dest & 0b111));
        self.code.extend(src.to_le_bytes());
    }

    pub fn mov_rm8_imm8(&mut self, rm8: AddressingMode, imm8: u8) {
        // Opcode: C6 /0 ib
        // Instruction: MOV r/m8, imm8
        // Op/En: MI (ModRM:r/m (w), imm8)
        // Description: Move imm8 to r/m8.

        self.rex(false, false, rm8.rex_x(), rm8.rex_b());
        self.opcode(0xc6);
        self.code.push(rm8.mod_r_m(0));
        self.code.extend(rm8.sib());
        self.code.extend(rm8.displacement8());
        self.code.extend(rm8.displacement32().iter().flatten());
        self.code.push(imm8);
    }

    pub fn mov_r8_rm8(&mut self, dest: ByteRegister, src: AddressingMode) {
        // Opcode: 8A /r
        // Instruction: MOV r8, r/m8
        // Op/En: RM (ModRM:reg (w), ModRM:r/m (r))
        // Description: Move r/m8 to r8.

        self.rex(false, dest as u8 & 0b1000 != 0, src.rex_x(), src.rex_b());
        self.opcode(0x8a);
        self.code.push(src.mod_r_m(dest as u8));
        self.code.extend(src.sib());
        self.code.extend(src.displacement8());
        self.code.extend(src.displacement32().iter().flatten());
    }

    pub fn mov_rm8_r8(&mut self, dest: AddressingMode, src: ByteRegister) {
        // Opcode: 88 /r
        // Instruction: MOV r/m8, r8
        // Op/En: MR (ModRM:r/m (w), ModRM:reg (r))
        // Description: Move r8 to r/m8.

        self.rex(false, src as u8 & 0b1000 != 0, dest.rex_x(), dest.rex_b());
        self.opcode(0x88);
        self.code.push(dest.mod_r_m(src as u8));
        self.code.extend(dest.sib());
        self.code.extend(dest.displacement8());
        self.code.extend(dest.displacement32().iter().flatten());
    }

    pub fn movzx_r32_rm8(&mut self, dest: QwordRegister, src: AddressingMode) {
        // Opcode: OF B6 /r
        // Instruction: MOVZX r32, r/m8
        // Op/En: RM (ModRM:reg (w), ModRM:r/m (r))
        // Description: Move byte to doubleword with zero-extension.

        self.rex(false, dest as u8 & 0b1000 != 0, src.rex_x(), src.rex_b());
        self.opcode(0x0f);
        self.opcode(0xb6);
        self.code.push(src.mod_r_m(dest as u8));
        self.code.extend(src.sib());
        self.code.extend(src.displacement8());
        self.code.extend(src.displacement32().iter().flatten());
    }

    pub fn cmp_rm8_imm8(&mut self, rm8: AddressingMode, imm8: u8) {
        // Opcode: 80 /7 ib
        // Instruction: CMP r/m8, imm8
        // Op/En: MI (ModRM:r/m (r), imm8)
        // Description: Compare imm8 with r/m8.

        self.rex(false, false, rm8.rex_x(), rm8.rex_b());
        self.opcode(0x80);
        self.code.push(rm8.mod_r_m(7));
        self.code.extend(rm8.sib());
        self.code.extend(rm8.displacement8());
        self.code.extend(rm8.displacement32().iter().flatten());
        self.code.push(imm8);
    }

    pub fn je_rel8(&mut self, rel8: i8) {
        // Opcode: 74 cb
        // Instruction: JE rel8
        // Op/En: D (Offset)
        // Description: Jump short if equal (ZF=1).

        self.opcode(0x74);
        self.code.push(rel8 as u8);
    }

    pub fn je_rel32(&mut self, rel32: i32) {
        // Opcode: 0F 84 cd
        // Instruction: JE rel32
        // Op/En: D (Offset)
        // Description: Jump near if equal (ZF=1).

        self.opcode(0x0f);
        self.opcode(0x84);
        self.code.extend(rel32.to_le_bytes());
    }

    pub fn set_je_rel8(&mut self, addr: usize, rel8: i8) {
        self.code[addr + 1] = rel8 as u8;
    }

    pub fn set_je_rel32(&mut self, addr: usize, rel32: i32) {
        self.code[addr + 2..addr + 6].copy_from_slice(&rel32.to_le_bytes());
    }

    pub fn jne_rel8(&mut self, rel8: i8) {
        // Opcode: 75 cb
        // Instruction: JNE rel8
        // Op/En: D (Offset)
        // Description: Jump short if not equal (ZF=0).

        self.opcode(0x75);
        self.code.push(rel8 as u8);
    }

    pub fn jne_rel32(&mut self, rel32: i32) {
        // Opcode: 0F 85 cd
        // Instruction: JNE rel32
        // Op/En: D (Offset)
        // Description: Jump near if not equal (ZF=0).

        self.opcode(0x0f);
        self.opcode(0x85);
        self.code.extend(rel32.to_le_bytes());
    }

    pub fn set_jne_rel8(&mut self, addr: usize, rel8: i8) {
        self.code[addr + 1] = rel8 as u8;
    }

    pub fn set_jne_rel32(&mut self, addr: usize, rel32: i32) {
        self.code[addr + 2..addr + 6].copy_from_slice(&rel32.to_le_bytes());
    }

    pub fn add_rm8_imm8(&mut self, rm8: AddressingMode, imm8: u8) {
        // Opcode: 80 /0 ib
        // Instruction: ADD r/m8, imm8
        // Op/En: MI (ModRM:r/m (r, w), imm8)
        // Description: Add imm8 to r/m8.

        self.rex(false, false, rm8.rex_x(), rm8.rex_b());
        self.opcode(0x80);
        self.code.push(rm8.mod_r_m(0));
        self.code.extend(rm8.sib());
        self.code.extend(rm8.displacement8());
        self.code.extend(rm8.displacement32().iter().flatten());
        self.code.push(imm8);
    }

    pub fn add_rm64_imm8(&mut self, rm64: AddressingMode, imm8: u8) {
        // Opcode: REX.W + 83 /0 ib
        // Instruction: ADD r/m64, imm8
        // Op/En: MI (ModRM:r/m (r, w), imm8)
        // Description: Add sign-extended imm8 to r/m64.

        self.rex(true, false, rm64.rex_x(), rm64.rex_b());
        self.opcode(0x83);
        self.code.push(rm64.mod_r_m(0));
        self.code.extend(rm64.sib());
        self.code.extend(rm64.displacement8());
        self.code.extend(rm64.displacement32().iter().flatten());
        self.code.push(imm8);
    }

    pub fn add_rm8_r8(&mut self, rm8: AddressingMode, r8: ByteRegister) {
        // Opcode: 00 /r
        // Instruction: ADD r/m8, r8
        // Op/En: MR (ModRM:r/m (r, w), ModRM:reg (r))
        // Description: Add r8 to r/m8.

        self.rex(false, r8 as u8 & 0b1000 != 0, rm8.rex_x(), rm8.rex_b());
        self.opcode(0x00);
        self.code.push(rm8.mod_r_m(r8 as u8));
        self.code.extend(rm8.sib());
        self.code.extend(rm8.displacement8());
        self.code.extend(rm8.displacement32().iter().flatten());
    }

    pub fn sub_rm8_imm8(&mut self, rm8: AddressingMode, imm8: u8) {
        // Opcode: 80 /5 ib
        // Instruction: SUB r/m8, imm8
        // Op/En: MI (ModRM:r/m (r, w), imm8)
        // Description: Subtract imm8 from r/m8.

        self.rex(false, false, rm8.rex_x(), rm8.rex_b());
        self.opcode(0x80);
        self.code.push(rm8.mod_r_m(5));
        self.code.extend(rm8.sib());
        self.code.extend(rm8.displacement8());
        self.code.extend(rm8.displacement32().iter().flatten());
        self.code.push(imm8);
    }

    pub fn sub_rm64_imm8(&mut self, rm64: AddressingMode, imm8: u8) {
        // Opcode: REX.W + 83 /5 ib
        // Instruction: SUB r/m64, imm8
        // Op/En: MI (ModRM:r/m (r, w), imm8)
        // Description: Subtract sign-extended imm8 from r/m64.

        self.rex(true, false, rm64.rex_x(), rm64.rex_b());
        self.opcode(0x83);
        self.code.push(rm64.mod_r_m(5));
        self.code.extend(rm64.sib());
        self.code.extend(rm64.displacement8());
        self.code.extend(rm64.displacement32().iter().flatten());
        self.code.push(imm8);
    }

    pub fn sub_rm8_r8(&mut self, rm8: AddressingMode, r8: ByteRegister) {
        // Opcode: 28 /r
        // Instruction: SUB r/m8, r8
        // Op/En: MR (ModRM:r/m (r, w), ModRM:reg (r))
        // Description: Subtract r8 from r/m8.

        self.rex(false, r8 as u8 & 0b1000 != 0, rm8.rex_x(), rm8.rex_b());
        self.opcode(0x28);
        self.code.push(rm8.mod_r_m(r8 as u8));
        self.code.extend(rm8.sib());
        self.code.extend(rm8.displacement8());
        self.code.extend(rm8.displacement32().iter().flatten());
    }

    pub fn mul_rm8(&mut self, rm8: AddressingMode) {
        // Opcode: F6 /4
        // Instruction: MUL r/m8
        // Op/En: M (ModRM:r/m (r))
        // Description: Unsigned multiply (AX := AL * r/m8).

        self.rex(false, false, rm8.rex_x(), rm8.rex_b());
        self.opcode(0xf6);
        self.code.push(rm8.mod_r_m(4));
        self.code.extend(rm8.sib());
        self.code.extend(rm8.displacement8());
        self.code.extend(rm8.displacement32().iter().flatten());
    }

    pub fn neg_rm8(&mut self, rm8: AddressingMode) {
        // Opcode: F6 /3
        // Instruction: NEG r/m8
        // Op/En: M (ModRM:r/m (r, w))
        // Description: Two's complement negate r/m8.

        self.rex(false, false, rm8.rex_x(), rm8.rex_b());
        self.opcode(0xf6);
        self.code.push(rm8.mod_r_m(3));
        self.code.extend(rm8.sib());
        self.code.extend(rm8.displacement8());
        self.code.extend(rm8.displacement32().iter().flatten());
    }

    // pub fn mov_r64_r64(&mut self, dest: QwordRegister, src: QwordRegister) {
    //     // Opcode: REX.W + 89 /r
    //     // Instruction: MOV r/m64, r64
    //     // Op/En: MR (ModRM:r/m (w), ModRM:reg (r))
    //     // Description: Move r64 to r/m64.

    //     let dest = dest as u8; // -> ModRM:r/m
    //     let src = src as u8; // -> ModRM:reg

    //     self.rex(true, src & 0b1000 != 0, false, dest & 0b1000 != 0);
    //     self.opcode(0x89);
    //     self.mod_r_m(0b11, src, dest);
    // }

    // pub fn call_absolute_indirect_reg(&mut self, reg: QwordRegister) {
    //     // Opcode: FF /2
    //     // Instruction: CALL r/m64
    //     // Op/En: M (ModRM:r/m (r))
    //     // Description: Call near, absolute indirect, address given in r/m64.

    //     let reg = reg as u8; // -> ModRM:r/m

    //     self.rex(false, false, false, reg & 0b1000 != 0);
    //     self.opcode(0xff);
    //     self.mod_r_m(0b11, 2, reg);
    // }

    // pub fn inc_byte_mem(&mut self, addr:
    // }

    // pub fn inc_qword(&mut self, reg: QwordRegister) {
    //     // Opcode: REX.W + FF /0
    //     // Instruction: INC r/m64
    //     // Op/En: M (ModRM:r/m (r, w))
    //     // Description: Increment r/m quadword by 1.

    //     let reg = reg as u8; // -> ModRM:r/m

    //     self.rex(true, false, false, reg & 0b1000 != 0);
    //     self.opcode(0xff);
    //     self.mod_r_m(0b11, 0, reg);
    // }

    // pub fn dec_byte(&mut self, reg: ByteRegister) {
    //     // Opcode: FE /1
    //     // Instruction: DEC r/m8
    //     // Op/En: M (ModRM:r/m (r, w))
    //     // Description: Decrement r/m8 by 1.

    //     let reg = reg as u8; // -> ModRM:r/m

    //     self.rex(false, false, false, reg & 0b1000 != 0);
    //     self.opcode(0xfe);
    //     self.mod_r_m(0b11, 1, reg);
    // }

    // pub fn dec_qword(&mut self, reg: QwordRegister) {
    //     // Opcode: REX.W + FF /1
    //     // Instruction: DEC r/m64
    //     // Op/En: M (ModRM:r/m (r, w))
    //     // Description: Decrement r/m64 by 1.

    //     let reg = reg as u8; // -> ModRM:r/m

    //     self.rex(true, false, false, reg & 0b1000 != 0);
    //     self.opcode(0xff);
    //     self.mod_r_m(0b11, 1, reg);
    // }
}

#[test]
fn test() {
    let mut assembler = Assembler::new();
    assembler.mov_rm64_r64(
        AddressingMode::Register {
            reg: QwordRegister::Rbp,
        },
        QwordRegister::Rsp,
    );
    println!("{:x?}", assembler.code);
}
