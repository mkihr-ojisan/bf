#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    /// `+`
    Increment,
    /// `-`
    Decrement,
    /// `>`
    PointerIncrement,
    /// `<`
    PointerDecrement,
    /// `.`
    PutChar,
    /// `,`
    GetChar,
    /// `[...]`
    Loop(Vec<Instruction>),

    /// ポインタが指す値に指定した値を加算する
    Add(u8),
    /// ポインタが指す値から指定した値を減算する
    Subtract(u8),
    /// ポインタが指す値を0に設定する
    SetZero,
    /// ポインタに指定した値を加算する
    PointerAdd(usize),
    /// ポインタに指定した値を減算する
    PointerSubtract(usize),
    /// ポインタが指す値に指定した位置の値を加算する。位置はポインタの位置からの相対位置で指定する
    AddValueAt(isize),
    /// ポインタが指す値から指定した位置の値を減算する。位置はポインタの位置からの相対位置で指定する
    SubtractValueAt(isize),
    /// ポインタが指す値に指定した位置の値を指定した値を乗算した値を加算する。位置はポインタの位置からの相対位置で指定する
    AddValueMultipliedBy(u8, isize),
    /// ポインタが指す値から指定した位置の値を指定した値を乗算した値を減算する。位置はポインタの位置からの相対位置で指定する
    SubtractValueMultipliedBy(u8, isize),
    /// ポインタが指す値の符号を反転する
    Negate,
    /// ポインタが指す値が0でない場合に指定した命令を実行する
    IfNotZero(Vec<Instruction>),
}
