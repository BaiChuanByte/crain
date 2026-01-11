use crate::cell::BfCell;
use crate::error::ParseError;
use crate::ir::BfIr;

impl<T> BfIr<T>
where
    T: BfCell,
{
    /// Parse the brainfuck code.
    ///
    /// Parse the brainfuck code, convert it into `BfIr` intermediate code.
    ///
    /// # Errors
    ///
    /// The function will return a `ParseError` if:
    /// 1. An error occurs during an IO operation.
    /// 2. A syntax error in the brainfuck code (such as mismatched brackets).
    ///
    /// # Examples
    /// ```rust
    /// use std::io::BufReader;
    /// use crain::ir::BfIr::{self, *};
    /// use crain::cell::Cell8;
    ///
    /// let code = BufReader::new("+-<>,.[]".as_bytes());
    /// let ic = BfIr::<Cell8>::parse(code)?;
    ///
    /// assert_eq!(
    ///     ic,
    ///     vec![AddCell(1), SubCell(1), LeftShift(1), RightShift(1),
    ///     Input, Output, Jz(7), Jnz(6)]
    /// );
    /// # Ok::<(), crain::ParseError>(())
    /// ```
    pub fn parse(source_codes: impl std::io::BufRead) -> Result<Vec<Self>, ParseError> {
        let mut codes: Vec<Self> = vec![];
        let mut stack: Vec<(usize, usize, usize)> = vec![]; // (pos, line, row)

        let mut line = 1usize;
        let mut row = 0usize;

        for byte in source_codes.bytes() {
            let byte = byte.map_err(|e| ParseError::IO { source: e })?;

            if byte == b'\n' {
                line += 1;
                row = 0;
                continue;
            }
            row += 1;

            macro_rules! wrapping_folding {
                ($codes:expr, $code:path) => {
                    if let Some($code(n)) = $codes.last_mut() {
                        n.add(T::ONE);
                    } else {
                        $codes.push($code(T::ONE));
                    }
                };
            }

            macro_rules! unwrapping_folding {
                ($codes:expr, $code:path) => {
                    if let Some($code(n)) = $codes.last_mut() {
                        if *n == usize::MAX {
                            $codes.push($code(1usize));
                        } else {
                            *n += 1;
                        }
                    } else {
                        $codes.push($code(1usize));
                    }
                };
            }

            match byte {
                b'+' => wrapping_folding!(codes, BfIr::AddCell),
                b'-' => wrapping_folding!(codes, BfIr::SubCell),
                b'<' => unwrapping_folding!(codes, BfIr::LeftShift),
                b'>' => unwrapping_folding!(codes, BfIr::RightShift),
                b',' => codes.push(BfIr::Input),
                b'.' => codes.push(BfIr::Output),

                b'[' => {
                    stack.push((codes.len(), line, row));
                    codes.push(BfIr::Jz(usize::MAX));
                }

                b']' => {
                    if let Some((pos, _, _)) = stack.pop() {
                        codes[pos] = BfIr::Jz(codes.len());
                        codes.push(BfIr::Jnz(pos));
                    } else {
                        return Err(ParseError::MismatchedBracket {
                            bracket: ']',
                            line,
                            row,
                        });
                    }
                }

                _ => {}
            }
        }

        if let Some((_, line, row)) = stack.pop() {
            return Err(ParseError::MismatchedBracket {
                bracket: '[',
                line,
                row,
            });
        }

        Ok(codes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::Cell8;
    use std::io::BufReader;

    #[test]
    fn test_bfcode_parse() {
        let error_func = |e| panic!("Parse Error: {e}");
        let ok_func = |v| v;
        let new_frame = |s: &str| {
            BfIr::<Cell8>::parse(BufReader::new(s.as_bytes())).map_or_else(error_func, ok_func)
        };

        assert_eq!(
            new_frame("+-<>,.[]"),
            vec![
                BfIr::AddCell(1),
                BfIr::SubCell(1),
                BfIr::LeftShift(1),
                BfIr::RightShift(1),
                BfIr::Input,
                BfIr::Output,
                BfIr::Jz(7),
                BfIr::Jnz(6),
            ]
        );

        assert_eq!(
            new_frame("-<<<+++--><++>>>"),
            vec![
                BfIr::SubCell(1),
                BfIr::LeftShift(3),
                BfIr::AddCell(3),
                BfIr::SubCell(2),
                BfIr::RightShift(1),
                BfIr::LeftShift(1),
                BfIr::AddCell(2),
                BfIr::RightShift(3),
            ]
        );

        assert_eq!(
            new_frame("[[][[[][[]][]]]]"),
            vec![
                BfIr::Jz(15),  // 0
                BfIr::Jz(2),   // 1
                BfIr::Jnz(1),  // 2
                BfIr::Jz(14),  // 3
                BfIr::Jz(13),  // 4
                BfIr::Jz(6),   // 5
                BfIr::Jnz(5),  // 6
                BfIr::Jz(10),  // 7
                BfIr::Jz(9),   // 8
                BfIr::Jnz(8),  // 9
                BfIr::Jnz(7),  // 10
                BfIr::Jz(12),  // 11
                BfIr::Jnz(11), // 12
                BfIr::Jnz(4),  // 13
                BfIr::Jnz(3),  // 14
                BfIr::Jnz(0),  // 15
            ]
        );
    }
}
