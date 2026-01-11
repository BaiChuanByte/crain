use crate::cell::BfCell;
use crate::error::ParseError;
use crate::ir::BfIr::{self, AddCell, SubCell, LeftShift, RightShift, Input, Output, Jz, Jnz};

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
                b'+' => wrapping_folding!(codes, AddCell),
                b'-' => wrapping_folding!(codes, SubCell),
                b'<' => unwrapping_folding!(codes, LeftShift),
                b'>' => unwrapping_folding!(codes, RightShift),
                b',' => codes.push(Input),
                b'.' => codes.push(Output),

                b'[' => {
                    stack.push((codes.len(), line, row));
                    codes.push(Jz(usize::MAX));
                }

                b']' => {
                    if let Some((pos, _, _)) = stack.pop() {
                        codes[pos] = Jz(codes.len());
                        codes.push(Jnz(pos));
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
                AddCell(1),
                SubCell(1),
                LeftShift(1),
                RightShift(1),
                Input,
                Output,
                Jz(7),
                Jnz(6),
            ]
        );

        assert_eq!(
            new_frame("-<<<+++--><++>>>"),
            vec![
                SubCell(1),
                LeftShift(3),
                AddCell(3),
                SubCell(2),
                RightShift(1),
                LeftShift(1),
                AddCell(2),
                RightShift(3),
            ]
        );

        assert_eq!(
            new_frame("[[][[[][[]][]]]]"),
            vec![
                Jz(15),  // 0
                Jz(2),   // 1
                Jnz(1),  // 2
                Jz(14),  // 3
                Jz(13),  // 4
                Jz(6),   // 5
                Jnz(5),  // 6
                Jz(10),  // 7
                Jz(9),   // 8
                Jnz(8),  // 9
                Jnz(7),  // 10
                Jz(12),  // 11
                Jnz(11), // 12
                Jnz(4),  // 13
                Jnz(3),  // 14
                Jnz(0),  // 15
            ]
        );
    }
}
