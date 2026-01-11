use clap::ValueEnum;
use o2o::o2o;

use crate::bfsetting;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
#[derive(o2o)]
#[map_owned(bfsetting::CellType)]
pub enum CellType {
    /// 8-bit Cell.
    #[default]
    #[value(name = "8")]
    Cell8,
    /// 16-bit Cell.
    #[value(name = "16")]
    Cell16,
    /// 32-bit Cell.
    #[value(name = "32")]
    Cell32,
    /// 64-bit Cell.
    #[value(name = "64")]
    Cell64,
    /// 128-bit Cell.
    #[value(name = "128")]
    Cell128,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
#[derive(o2o)]
#[map_owned(bfsetting::EofValue)]
pub enum EofValue {
    /// Convert EOF to 0.
    #[default]
    Zero,
    /// Convert EOF to -1 (for example, 0xFF in an 8-bit cell).
    NegativeOne,
    /// Do not write any value, keeping the cell unchanged.
    NoWrite,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
#[derive(o2o)]
#[map_owned(bfsetting::Endian)]
pub enum Endian {
    /// Little-endian, where the least significant byte is stored first.
    #[default]
    Little,
    /// Big-endian, where the most significant byte is stored first.
    Big,
    /// Native-endian, which uses the endianness of the host machine.
    Native,
}
