use std::fs::File;
use std::io::{BufReader, Cursor};

use super::*;
use crate::bfir::BfCode::*;
use crate::celltype::*;
use crate::error::{InterpError, VmError};

/// The virtual machine used to run Brainfuck programs.
///
/// Brainfuck virtual machine. The construction is similar to the Turing machine, with a tape and a pointer.
/// It is used to run Brainfuck programs.
pub struct BfVm<T: BfCell> {
    array: Vec<T>,
    ptr: usize,
}

impl<T: BfCell> BfVm<T> {
    /// News a BfVm.
    ///
    /// # Panics
    /// it will cause a panic if you set size to zero, or set ptr to beyond the range of the array.
    ///
    /// # Examples
    /// ```rust
    /// crain::interp::BfVm::<crain::celltype::Cell8>::new(30000, 0);
    /// ```
    pub fn new(size: usize, ptr: usize) -> Self {
        if size == 0 {
            panic!("ValueError: Illegal parameter \"size\"")
        }
        if ptr >= size {
            panic!("ValueError: Illegal parameter \"ptr\"")
        }
        BfVm {
            array: vec![T::ZERO; size],
            ptr,
        }
    }

    /// Step through a BfFrame.
    ///
    /// Run a BfFrame, but only for a single step.
    ///
    /// # Failures
    /// The function will return a `VmError` if:
    /// 1. An error occurs during an IO operation.
    /// 2. An operation not allowed by BfVm is performed.
    ///
    /// # Examples
    /// ```rust
    /// use std::io::Cursor;
    /// use crain::celltype::Cell8;
    /// use crain::interp::{BfFrame, BfVm};
    ///
    /// let mut frame = BfFrame::<Cell8>::new(Cursor::new(
    ///     "+.".as_bytes().to_vec()
    /// ))?;
    /// let mut vm = BfVm::new(1, 0);
    ///
    /// let mut input = Cursor::new("".as_bytes().to_vec());
    /// let mut output = Cursor::new("".as_bytes().to_vec());
    ///
    /// vm.simple_step(&mut input, &mut output, &mut frame)?;
    /// vm.simple_step(&mut input, &mut output, &mut frame)?;
    ///
    /// assert_eq!(*output.get_ref(), "\x01".as_bytes().to_vec());
    /// # Ok::<(), crain::InterpError>(())
    /// ```
    pub fn simple_step(
        &mut self,
        bfinput: &mut impl std::io::Read,
        bfoutput: &mut impl std::io::Write,
        frame: &mut BfFrame<T>,
    ) -> Result<(), VmError> {
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
                    return Err(VmError::PointerOverflow {
                        info: "Overflow on left".to_string(),
                    });
                }
            }
            RightShift(n) => {
                if len - *ptr > n {
                    *ptr += n;
                } else {
                    return Err(VmError::PointerOverflow {
                        info: "Overflow on right".to_string(),
                    });
                }
            }

            Input => cell
                .input(bfinput)
                .map_err(|e| VmError::IO { source: e })?,
            Output => cell
                .output(bfoutput)
                .map_err(|e| VmError::IO { source: e })?,

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

    /// Run a BfFrame.
    ///
    /// Run the BfFrame until all the remaining code has been executed.
    ///
    /// # Failures
    /// The function will return a `VmError` if:
    /// 1. An error occurs during an IO operation.
    /// 2. An operation not allowed by BfVm is performed.
    ///
    /// # Examples
    /// ```rust
    /// use std::io::Cursor;
    /// use crain::celltype::Cell8;
    /// use crain::interp::{BfFrame, BfVm};
    ///
    /// let mut frame = BfFrame::<Cell8>::new(Cursor::new(
    ///     "+++++++++++++[->+++++<]>.".as_bytes().to_vec()
    /// ))?;
    /// let mut vm = BfVm::new(2, 0);
    ///
    /// let mut input = Cursor::new("".as_bytes().to_vec());
    /// let mut output = Cursor::new("".as_bytes().to_vec());
    ///
    /// vm.run(&mut input, &mut output, &mut frame)?;
    ///
    /// assert_eq!(*output.get_ref(), "A".as_bytes().to_vec());
    /// # Ok::<(), crain::InterpError>(())
    /// ```
    pub fn run(
        &mut self,
        bfinput: &mut impl std::io::Read,
        bfoutput: &mut impl std::io::Write,
        frame: &mut BfFrame<T>,
    ) -> Result<(), VmError> {
        while *frame.pc() < frame.codes().len() {
            self.simple_step(bfinput, bfoutput, frame)?;
        }

        Ok(())
    }
}

/// Eval a brainfuck code file.
///
/// Open a file by name and eval it.
///
/// # Failures
/// The function will return a `VmError` if:
/// 1. An error occurs during an IO operation.
/// 2. A syntax error in the brainfuck code (such as mismatched brackets).
/// 3. An operation not allowed by BfVm is performed.
///
/// # Example
/// ```rust, no_run
/// crain::interp::eval_file("your_file_name.bf", 30000, 0);
/// ```
pub fn eval_file(name: String, size: usize, ptr: usize) -> Result<(), InterpError> {
    let f = File::open(name)?;
    let mut code = BfFrame::<Cell8>::new(BufReader::new(f))?;
    let mut vm = BfVm::<Cell8>::new(size, ptr);
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    vm.run(&mut stdin, &mut stdout, &mut code)?;

    Ok(())
}

/// Eval a brainfuck code string.
///
/// Get a string and eval it.
///
/// # Failures
/// The function will return a `VmError` if:
/// 1. An error occurs during an IO operation.
/// 2. A syntax error in the brainfuck code (such as mismatched brackets).
/// 3. An operation not allowed by BfVm is performed.
///
/// # Example
/// ```rust
/// // print "A"
/// crain::interp::eval_string("\"+++++++++++++[->+++++<]>.bf\"", 30000, 0);
/// ```
pub fn eval_string(code: String, size: usize, ptr: usize) -> Result<(), InterpError> {
    let mut code = BfFrame::<Cell8>::new(Cursor::new(code))?;
    let mut vm = BfVm::<Cell8>::new(size, ptr);
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    vm.run(&mut stdin, &mut stdout, &mut code)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_runner() {
        let mut vm = BfVm::<Cell8>::new(10, 0);
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut output = Cursor::new(Vec::<u8>::new());

        let error_func = |e| panic!("Parse Error: {e}");
        let ok_func = |v| v;
        let new_frame = |s: &str| {
            BfFrame::<Cell8>::new(BufReader::new(s.as_bytes())).map_or_else(error_func, ok_func)
        };

        let mut frame = new_frame(
            "
             ++++  +++++    +[   - >    +
            +    + +    +  +  >  - ->   +
            +      +    < <    < ] > +  +
            +      ++++.  [->++> + <  < ]
            >    . >   .  <    - - -   --
             ----  .    + +    + + +    .
            ",
        );

        vm.run(&mut input, &mut output, &mut frame)
            .map_or_else(|e| panic!("Vm Error: {e}"), |v| v);

        assert_eq!(*output.get_ref(), Vec::<Cell8>::from("Crain"));
    }
}
