use crate::bfsetting::*;

use super::*;

/// 8-bits Cell. It is essentially a wrapper around u8.
pub type Cell8 = u8;

/// 16-bits Cell. It is essentially a wrapper around u16.
pub type Cell16 = u16;

/// 32-bits Cell. It is essentially a wrapper around u32.
pub type Cell32 = u32;

/// 64-bits Cell. It is essentially a wrapper around u64.
pub type Cell64 = u64;

/// 128-bits Cell. It is essentially a wrapper around u128.
pub type Cell128 = u128;

macro_rules! impl_bfcell {
    ($t:ty) => {
        impl BfCell for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const BITS: u32 = 8 * (std::mem::size_of::<Self>() as u32);

            fn add(&mut self, rhs: Self) {
                *self = self.wrapping_add(rhs);
            }
            fn sub(&mut self, rhs: Self) {
                *self = self.wrapping_sub(rhs);
            }

            fn iszero(self) -> bool {
                self == 0
            }

            fn input(&mut self, bfinput: &mut impl std::io::BufRead, setting: &BfSetting) -> Result<(), std::io::Error> {
                let mut bytes = [0u8];
                let mut no_write = false;
                let peeked;

                bfinput.read_exact(&mut bytes).or_else(|e| {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        match setting.eof_value {
                            EofValue::Zero => bytes[0] = 0,
                            EofValue::NegativeOne => bytes[0] = 0xFF,
                            EofValue::NoWrite => no_write = true,
                        }
                        Ok(())
                    } else {
                        Err(e)
                    }
                })?;
                if no_write {
                    return Ok(());
                }
                let mut byte = bytes[0];

                if setting.translate_newline {
                    if byte == b'\r' {
                        peeked = bfinput.fill_buf().map_or_else(|e| {
                            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                                Ok(None)
                            } else {
                                Err(e)
                            }
                        }, |buf| {
                            if buf.is_empty() {
                                Ok(None)
                            } else {
                                Ok(Some(buf[0]))
                            }
                        })?;

                        if let Some(peeked) = peeked {
                            byte = peeked;
                            bfinput.consume(1); // Consume the '\n' byte what we peeked.
                        }
                    }
                }

                *self = byte.into();
                Ok(())
            }

            fn output(self, bfoutput: &mut impl std::io::Write, setting: &BfSetting) -> Result<(), std::io::Error> {
                let buf = match setting.endian {
                    Endian::Little => self.to_le_bytes(),
                    Endian::Big => self.to_be_bytes(),
                    Endian::Native => self.to_ne_bytes(),
                };

                if setting.translate_newline {
                    let mut i = 0;
                    let size = buf.len();
                    let mut translate_buf = Vec::with_capacity(size);

                    while i < size {
                        let byte = &buf[i];
                        if *byte == b'\n' {
                            translate_buf.push(b'\r');
                            translate_buf.push(b'\n');
                            i += 2;
                        } else {
                            translate_buf.push(*byte);
                            i += 1;
                        }
                    }
                    bfoutput.write_all(translate_buf.as_ref())?;
                } else {
                    bfoutput.write_all(&buf)?
                }

                bfoutput.flush()?;

                Ok(())
            }
        }
    };
}

impl_bfcell!(Cell8);
impl_bfcell!(Cell16);
impl_bfcell!(Cell32);
impl_bfcell!(Cell64);
impl_bfcell!(Cell128);
