use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::PathBuf;

use super::BfFrame;
use crate::bfsetting::BfSetting;
use crate::cell::{BfCell, Cell8};
use crate::error::{InterpError, VmError};
use crate::ir::BfIr;

/// The virtual machine used to run Brainfuck programs.
///
/// Brainfuck virtual machine. The construction is similar to the Turing machine, with a tape and a pointer.
/// It is used to run Brainfuck programs.
pub struct BfVm<T: BfCell> {
    array: Vec<T>,
    ptr: usize,
    setting: BfSetting,
}

impl<T: BfCell> BfVm<T> {
    /// News a `BfVm`.
    ///
    /// # Panics
    /// it will cause a panic if you set size to zero, or set ptr to beyond the range of the array.
    ///
    /// # Examples
    /// ```rust
    /// use crain::*;
    /// BfVm::<Cell8>::new(BfSetting::default());
    /// ```
    pub fn new(setting: BfSetting) -> Self {
        assert!(
            (setting.size != 0),
            "ValueError: Illegal parameter \"size\""
        );
        assert!(
            (setting.ptr < setting.size),
            "ValueError: Illegal parameter \"ptr\""
        );
        Self {
            array: vec![T::ZERO; setting.size],
            ptr: setting.ptr,
            setting,
        }
    }

    /// Step through a `BfFrame`.
    ///
    /// Run a `BfFrame`, but only for a single step.
    ///
    /// # Errors
    /// The function will return a `VmError` if:
    /// 1. An error occurs during an IO operation.
    /// 2. An operation not allowed by `BfVm` is performed.
    ///
    /// # Examples
    /// ```rust
    /// use std::io::Cursor;
    /// use crain::BfSetting;
    /// use crain::cell::Cell8;
    /// use crain::interp::{BfFrame, BfVm};
    ///
    /// let mut frame = BfFrame::<Cell8>::new(Cursor::new(
    ///     "+.".as_bytes().to_vec()
    /// ))?;
    /// let mut vm = BfVm::new(BfSetting{
    ///     size: 1,
    ///     ..Default::default()
    /// });
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
        bfinput: &mut impl std::io::BufRead,
        bfoutput: &mut impl std::io::Write,
        frame: &mut BfFrame<T>,
    ) -> Result<(), VmError> {
        let len = self.array.len();
        let ptr = &mut self.ptr;
        let cell = &mut self.array[*ptr];

        match frame.codes()[*frame.pc()] {
            BfIr::AddCell(n) => cell.add(n),
            BfIr::SubCell(n) => cell.sub(n),

            BfIr::LeftShift(n) => {
                if *ptr >= n {
                    *ptr -= n;
                } else {
                    return Err(VmError::PointerOverflow {
                        info: "Overflow on left".to_string(),
                    });
                }
            }
            BfIr::RightShift(n) => {
                if len - *ptr > n {
                    *ptr += n;
                } else {
                    return Err(VmError::PointerOverflow {
                        info: "Overflow on right".to_string(),
                    });
                }
            }

            BfIr::Input => cell
                .input(bfinput, &self.setting)
                .map_err(|e| VmError::IO { source: e })?,
            BfIr::Output => cell
                .output(bfoutput, &self.setting)
                .map_err(|e| VmError::IO { source: e })?,

            BfIr::Jz(n) => {
                if cell.iszero() {
                    frame.jump(n);
                }
            }
            BfIr::Jnz(n) => {
                if !cell.iszero() {
                    frame.jump(n);
                }
            }
        }

        frame.jump(frame.pc() + 1);

        Ok(())
    }

    /// Run a `BfFrame`.
    ///
    /// Run the `BfFrame` until all the remaining code has been executed.
    ///
    /// # Errors
    /// The function will return a `VmError` if:
    /// 1. An error occurs during an IO operation.
    /// 2. An operation not allowed by `BfVm` is performed.
    ///
    /// # Examples
    /// ```rust
    /// use std::io::Cursor;
    /// use crain::BfSetting;
    /// use crain::cell::Cell8;
    /// use crain::interp::{BfFrame, BfVm};
    ///
    /// let mut frame = BfFrame::<Cell8>::new(Cursor::new(
    ///     "+++++++++++++[->+++++<]>.".as_bytes().to_vec()
    /// ))?;
    /// let mut vm = BfVm::new(BfSetting{
    ///     size: 2,
    ///     ..Default::default()
    /// });
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
        bfinput: &mut impl std::io::BufRead,
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
/// # Errors
/// The function will return a `VmError` if:
/// 1. An error occurs during an IO operation.
/// 2. A syntax error in the brainfuck code (such as mismatched brackets).
/// 3. An operation not allowed by `BfVm` is performed.
///
/// # Example
/// ```rust, no_run
/// crain::interp::eval_file(&"your_file_name.bf".to_string().into(), Default::default());
/// ```
pub fn eval_file(name: &PathBuf, setting: BfSetting) -> Result<(), InterpError> {
    let f = File::open(name)?;
    let mut code = BfFrame::<Cell8>::new(BufReader::new(f))?;
    let mut vm = BfVm::<Cell8>::new(setting);
    let mut stdin = BufReader::new(std::io::stdin());
    let mut stdout = std::io::stdout();

    vm.run(&mut stdin, &mut stdout, &mut code)?;

    Ok(())
}

/// Eval a brainfuck code string.
///
/// Get a string and eval it.
///
/// # Errors
/// The function will return a `VmError` if:
/// 1. An error occurs during an IO operation.
/// 2. A syntax error in the brainfuck code (such as mismatched brackets).
/// 3. An operation not allowed by `BfVm` is performed.
///
/// # Example
/// ```rust
/// // print "A"
/// crain::interp::eval_string("\"+++++++++++++[->+++++<]>.bf\"", Default::default());
/// ```
pub fn eval_string(code: &str, setting: BfSetting) -> Result<(), InterpError> {
    let mut code = BfFrame::<Cell8>::new(Cursor::new(code))?;
    let mut vm = BfVm::<Cell8>::new(setting);
    let mut stdin = BufReader::new(std::io::stdin());
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
        let mut vm = BfVm::<Cell8>::new(BfSetting {
            size: 4,
            ptr: 0,
            ..Default::default()
        });
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut output = Cursor::new(Vec::<u8>::new());

        let error_func = |e| panic!("Error: {e}");
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
            .unwrap_or_else(|e| panic!("Vm Error: {e}"));

        assert_eq!(*output.get_ref(), Vec::<u8>::from("Crain"));
    }
}
