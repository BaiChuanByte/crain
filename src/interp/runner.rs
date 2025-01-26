use std::fs::File;
use std::io::{BufReader, Cursor};

use super::*;
use crate::bfir::BFCode::*;
use crate::celltype::BFCell;
use crate::error::{InterpError, VMError};

pub struct BFVM<T: BFCell> {
    array: Vec<T>,
    ptr: usize,
}

impl<T: BFCell> BFVM<T> {
    fn new(size: usize, ptr: usize) -> Self {
        if size == 0 {
            panic!("ValueError: Illegal parameter \"size\"")
        }
        if ptr >= size {
            panic!("ValueError: Illegal parameter \"ptr\"")
        }
        BFVM {
            array: vec![T::zero(); size],
            ptr,
        }
    }

    fn simple_step(
        &mut self,
        bfinput: &mut impl std::io::Read,
        bfoutput: &mut impl std::io::Write,
        frame: &mut BFFrame<T>,
    ) -> Result<(), VMError> {
        let len = self.array.len();
        let ptr = &mut self.ptr;
        let cell = &mut self.array[*ptr];

        match frame.codes()[*frame.pc()] {
            AddCell(n) => cell.add(n),
            SubCell(n) => cell.sub(n),

            LeftShift(n) => {
                if *ptr >= n {
                    *ptr -= n;
                } else {
                    return Err(VMError::PointerOverflow {
                        info: "Overflow on left".to_string(),
                    });
                }
            }
            RightShift(n) => {
                if len - *ptr > n {
                    *ptr += n;
                } else {
                    return Err(VMError::PointerOverflow {
                        info: "Overflow on right".to_string(),
                    });
                }
            }

            Input => cell.input(bfinput).map_err(|e| VMError::IO { source: e })?,
            Output => cell
                .output(bfoutput)
                .map_err(|e| VMError::IO { source: e })?,

            Jz(n) => {
                if cell.iszero() {
                    frame.jump(n)
                }
            }
            Jnz(n) => {
                if !cell.iszero() {
                    frame.jump(n)
                }
            }
        }

        frame.jump(frame.pc() + 1);

        Ok(())
    }

    fn run(
        &mut self,
        bfinput: &mut impl std::io::Read,
        bfoutput: &mut impl std::io::Write,
        frame: &mut BFFrame<T>,
    ) -> Result<(), VMError> {
        while *frame.pc() < frame.codes().len() {
            self.simple_step(bfinput, bfoutput, frame)?;
        }

        Ok(())
    }
}

pub fn run_bf_file(name: String, size: usize, ptr: usize) -> Result<(), InterpError> {
    let f = File::open(name)?;
    let mut code = BFFrame::<u8>::new(BufReader::new(f))?;
    let mut vm = BFVM::<u8>::new(size, ptr);
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    vm.run(&mut stdin, &mut stdout, &mut code)?;

    Ok(())
}

pub fn run_bf_string(code: String, size: usize, ptr: usize) -> Result<(), InterpError> {
    let mut code = BFFrame::<u8>::new(Cursor::new(code))?;
    let mut vm = BFVM::<u8>::new(size, ptr);
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    vm.run(&mut stdin, &mut stdout, &mut code)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bfir::BFCode;
    use std::io::BufReader;

    #[test]
    fn test_runner() {
        let mut vm = BFVM::<u8>::new(10, 0);
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut output = Cursor::new(Vec::<u8>::new());

        let error_func = |e| panic!("Parse Error: {e}");
        let ok_func = |v| v;
        let new_frame = |s: &str| {
            BFFrame::<u8>::new(BufReader::new(s.as_bytes())).map_or_else(error_func, ok_func)
        };

        let mut frame = new_frame("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[-]++++++++++.[-]");

        vm.run(&mut input, &mut output, &mut frame)
            .map_or_else(|e| panic!("VM Error: {e}"), |v| v);

        assert_eq!(*output.get_ref(), Vec::<u8>::from("Crain\n"));
    }
}
