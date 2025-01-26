use super::*;

pub type Cell8 = u8;
pub type Cell16 = u16;
pub type Cell32 = u32;
pub type Cell64 = u64;
pub type Cell128 = u128;

macro_rules! impl_bfcell {
    ($t:ty) => {
        impl BFCell for $t {
            fn zero() -> Self {
                0
            }
            fn one() -> Self {
                1
            }

            fn add(&mut self, rhs: Self) {
                *self = self.wrapping_add(rhs);
            }
            fn sub(&mut self, rhs: Self) {
                *self = self.wrapping_sub(rhs);
            }

            fn iszero(self) -> bool {
                self == 0
            }

            fn input(&mut self, bfinput: &mut impl std::io::Read) -> Result<(), std::io::Error> {
                // compiler cannot know it
                const BYTES_COUNT: usize = (<$t>::BITS / 8) as usize;
                let mut buf = [0u8; BYTES_COUNT];

                bfinput.read(&mut buf)?;
                *self = <$t>::from_ne_bytes(buf);

                Ok(())
            }

            fn output(self, bfoutput: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                let buf = self.to_ne_bytes();

                bfoutput.write(&buf)?;
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
