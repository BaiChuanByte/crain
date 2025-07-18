use smart_default::SmartDefault;

#[derive(Debug, Eq, PartialEq, SmartDefault)]
pub struct BfSetting {
    /// The size of the memory array.
    #[default = 30000]
    pub size: usize,

    /// The initial pointer position.
    #[default = 0]
    pub ptr: usize,

    /// The value to if translate "\r\n" to "\n" (0x0A).
    #[default = cfg!(target_family = "windows")]
    pub translate_newline: bool,

    /// The value to set what EOF is converted to.
    #[default(EofValue::Zero)]
    pub eof_value: EofValue,

    /// The value to set which endian interprets the cell as character(s).
    #[default(Endian::Little)]
    pub endian: Endian,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EofValue {
    /// Convert EOF to 0.
    Zero,
    /// Convert EOF to -1 (for example, 0xFF in an 8-bit cell).
    NegativeOne,
    /// Do not write any value, keeping the cell unchanged.
    NoWrite,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endian {
    /// Little-endian, where the least significant byte is stored first.
    Little,
    /// Big-endian, where the most significant byte is stored first.
    Big,
    /// Native-endian, which uses the endianness of the host machine.
    Native,
}
