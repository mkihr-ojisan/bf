use super::QwordRegister;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// [rax]
    Indirect { reg: QwordRegister },
    /// [rip + 0x12345678]
    RipDisplacement32 { disp: i32 },
    /// [rax + 0x12]
    IndirectDisplacement8 { base: QwordRegister, disp: i8 },
    /// [rax + 0x12345678]
    IndirectDisplacement32 { base: QwordRegister, disp: i32 },
    /// [rax + rcx * 2]
    IndirectScaled {
        base: QwordRegister,
        index: Option<QwordRegister>,
        scale: AddressingScale,
    },
    /// [rax + rcx * 2 + 0x12]
    IndirectScaledDisplacement8 {
        base: QwordRegister,
        index: Option<QwordRegister>,
        scale: AddressingScale,
        disp: i8,
    },
    /// [rax + rcx * 2 + 0x12345678]
    IndirectScaledDisplacement32 {
        base: QwordRegister,
        index: Option<QwordRegister>,
        scale: AddressingScale,
        disp: i32,
    },
    /// [rax * 2 + 0x12345678]
    IndirectScaledBaseDisplacement32 {
        index: Option<QwordRegister>,
        scale: AddressingScale,
        disp: i32,
    },
    /// [rax * 2 + 0x12 + rbp]
    IndirectScaledBaseDisplacement8Rbp {
        index: Option<QwordRegister>,
        scale: AddressingScale,
        disp: i8,
    },
    /// [rax * 2 + 0x12345678 + rbp]
    IndirectScaledBaseDisplacement32Rbp {
        index: Option<QwordRegister>,
        scale: AddressingScale,
        disp: i32,
    },
    /// rax
    Register { reg: QwordRegister },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingScale {
    Scale1,
    Scale2,
    Scale4,
    Scale8,
}

impl AddressingMode {
    pub(super) fn mod_r_m(self, reg: u8) -> u8 {
        let mod_ = match self {
            AddressingMode::Indirect { .. }
            | AddressingMode::RipDisplacement32 { .. }
            | AddressingMode::IndirectScaled { .. }
            | AddressingMode::IndirectScaledBaseDisplacement32 { .. } => 0b00,
            AddressingMode::IndirectDisplacement8 { .. }
            | AddressingMode::IndirectScaledDisplacement8 { .. }
            | AddressingMode::IndirectScaledBaseDisplacement8Rbp { .. } => 0b01,
            AddressingMode::IndirectDisplacement32 { .. }
            | AddressingMode::IndirectScaledDisplacement32 { .. }
            | AddressingMode::IndirectScaledBaseDisplacement32Rbp { .. } => 0b10,
            AddressingMode::Register { .. } => 0b11,
        };

        let r_m = match self {
            AddressingMode::Indirect { reg } => {
                if reg == QwordRegister::Rsp || reg == QwordRegister::Rbp {
                    panic!(
                        "rsp and rbp are not usable as base register in indirect addressing mode"
                    );
                }
                reg as u8 & 0b111
            }
            AddressingMode::RipDisplacement32 { .. } => 0b101,
            AddressingMode::IndirectDisplacement8 { base, .. }
            | AddressingMode::IndirectDisplacement32 { base, .. } => {
                if base == QwordRegister::Rsp {
                    panic!("rsp is not usable as base register in indirect addressing mode with displacement");
                }
                base as u8 & 0b111
            }
            AddressingMode::IndirectScaled { .. }
            | AddressingMode::IndirectScaledDisplacement8 { .. }
            | AddressingMode::IndirectScaledDisplacement32 { .. }
            | AddressingMode::IndirectScaledBaseDisplacement32 { .. }
            | AddressingMode::IndirectScaledBaseDisplacement8Rbp { .. }
            | AddressingMode::IndirectScaledBaseDisplacement32Rbp { .. } => 0b100,
            AddressingMode::Register { reg } => reg as u8 & 0b111,
        };

        (mod_ << 6) | ((reg & 0b111) << 3) | r_m
    }

    pub(super) fn sib(self) -> Option<u8> {
        match self {
            AddressingMode::Indirect { .. }
            | AddressingMode::RipDisplacement32 { .. }
            | AddressingMode::IndirectDisplacement8 { .. }
            | AddressingMode::IndirectDisplacement32 { .. }
            | AddressingMode::Register { .. } => None,
            AddressingMode::IndirectScaled { base, index, scale }
            | AddressingMode::IndirectScaledDisplacement8 {
                base, index, scale, ..
            }
            | AddressingMode::IndirectScaledDisplacement32 {
                base, index, scale, ..
            } => {
                let ss = scale as u8;

                if index == Some(QwordRegister::Rsp) {
                    panic!("rsp is not usable as index register in SIB");
                }
                let index = index.unwrap_or(QwordRegister::Rsp) as u8 & 0b111;

                if base == QwordRegister::Rbp {
                    panic!("rsp is not usable as base register in SIB");
                }
                let base = base as u8 & 0b111;

                Some((ss << 6) | (index << 3) | base)
            }
            AddressingMode::IndirectScaledBaseDisplacement32 { index, scale, .. }
            | AddressingMode::IndirectScaledBaseDisplacement8Rbp { index, scale, .. }
            | AddressingMode::IndirectScaledBaseDisplacement32Rbp { index, scale, .. } => {
                if scale == AddressingScale::Scale8 {
                    panic!("scale 8 is not usable with indirect scaled base displacement addressing mode");
                }
                let ss = scale as u8;

                if index == Some(QwordRegister::Rsp) {
                    panic!("rsp is not usable as index register in SIB");
                }
                let index = index.unwrap_or(QwordRegister::Rsp) as u8 & 0b111;

                let base = QwordRegister::Rbp as u8 & 0b111;

                Some((ss << 6) | (index << 3) | base)
            }
        }
    }

    pub(super) fn displacement8(self) -> Option<u8> {
        match self {
            AddressingMode::IndirectDisplacement8 { disp, .. }
            | AddressingMode::IndirectScaledDisplacement8 { disp, .. }
            | AddressingMode::IndirectScaledBaseDisplacement8Rbp { disp, .. } => Some(disp as u8),
            _ => None,
        }
    }

    pub(super) fn displacement32(self) -> Option<[u8; 4]> {
        match self {
            AddressingMode::RipDisplacement32 { disp }
            | AddressingMode::IndirectDisplacement32 { disp, .. }
            | AddressingMode::IndirectScaledDisplacement32 { disp, .. }
            | AddressingMode::IndirectScaledBaseDisplacement32 { disp, .. }
            | AddressingMode::IndirectScaledBaseDisplacement32Rbp { disp, .. } => {
                Some(disp.to_le_bytes())
            }
            _ => None,
        }
    }

    pub(super) fn rex_x(self) -> bool {
        match self {
            AddressingMode::IndirectScaled { index, .. }
            | AddressingMode::IndirectScaledDisplacement8 { index, .. }
            | AddressingMode::IndirectScaledDisplacement32 { index, .. }
            | AddressingMode::IndirectScaledBaseDisplacement32 { index, .. }
            | AddressingMode::IndirectScaledBaseDisplacement8Rbp { index, .. }
            | AddressingMode::IndirectScaledBaseDisplacement32Rbp { index, .. } => {
                index.is_some() && index.unwrap() as u8 & 0b1000 != 0
            }
            _ => false,
        }
    }

    pub(super) fn rex_b(self) -> bool {
        match self {
            AddressingMode::Indirect { reg }
            | AddressingMode::IndirectDisplacement8 { base: reg, .. }
            | AddressingMode::IndirectDisplacement32 { base: reg, .. }
            | AddressingMode::Register { reg }
            | AddressingMode::IndirectScaled { base: reg, .. }
            | AddressingMode::IndirectScaledDisplacement8 { base: reg, .. }
            | AddressingMode::IndirectScaledDisplacement32 { base: reg, .. } => {
                reg as u8 & 0b1000 != 0
            }
            _ => false,
        }
    }
}
