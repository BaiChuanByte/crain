use crate::error::*;
use super::bfcode::BFCode;
use super::celltype::BFCell;


impl<T> BFCode<T>
    where T: BFCell {
    pub fn parse(source_codes: impl std::io::BufRead) -> Result<Vec<BFCode<T>>, ParseError> {
        let mut codes = vec![];
        let mut stack: Vec<(usize, usize, usize)> = vec![];

        let mut line = 1usize;
        let mut row = 0usize;

        macro_rules! wrapping_push {
            ($vector:expr, $code:path) => {
                if let Some($code(n)) = $vector.last() {
                    // Pre dereference reference "n" to end the lifetime of immutable
                    // reference and avoid conflicts with mutable reference "last"
                    let n = *n;

                    let last = $vector.last_mut().unwrap();
                    *last = $code(n + (T::one()));
                } else {
                    $vector.push($code(T::one()));
                }
            };
        }

        macro_rules! unwrapping_push {
            ($vector:expr, $code:path) => {
                if let Some($code(n)) = $vector.last() {
                    // Pre dereference reference "n" to end the lifetime of immutable
                    // reference and avoid conflicts with mutable reference "last"
                    let n = *n;

                    if n == usize::MAX {
                        $vector.push($code(1usize));
                    } else {
                        let last = $vector.last_mut().unwrap();
                        *last = $code(n + 1);
                    }
                } else {
                    $vector.push($code(1usize));
                }
            };
        }

        for byte in source_codes.bytes() {
            let byte = byte.map_err(|e| {
                ParseError::IO{source: e}
            })?;

            if byte == b'\n' { line += 1; row = 0; continue; }
            row += 1;

            match byte {
                b'+' => { wrapping_push!(codes, BFCode::<T>::AddCell) },
                b'-' => { wrapping_push!(codes, BFCode::<T>::SubCell) },
                b'<' => { unwrapping_push!(codes, BFCode::<T>::LeftShift) },
                b'>' => { unwrapping_push!(codes, BFCode::<T>::RightShift) },
                b',' => { codes.push(BFCode::<T>::Input) },
                b'.' => { codes.push(BFCode::<T>::Output) },
                b'[' => { stack.push( (codes.len(), line, row) );
                          codes.push( BFCode::<T>::Jz(usize::MAX) ); },
                b']' => {
                    if let Some(left_bracket_data) = stack.pop() {
                        codes[left_bracket_data.0] =
                            BFCode::<T>::Jz(codes.len());
                        codes.push(BFCode::<T>::Jnz(left_bracket_data.0));
                    } else {
                        return Err(ParseError::MismatchedBracket{
                            bracket: ']',
                            line,
                            row,
                        });
                    }
                },
                _ => {}
            }
        }

        if let Some(left_bracket_data) = stack.pop() {
            return Err(ParseError::MismatchedBracket{
                bracket: '[',
                line: left_bracket_data.1,
                row: left_bracket_data.2,
            });
        }

        Ok(codes)
    }
}



#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use crate::bfir::BFCode::*;
    use super::*;

    #[test]
    fn test_bfcode_parse() {
        let error_func = |e| panic!("Parse Error: {}", e);
        let ok_func    = |v| v;

        // test the parse function
        let source_codes = BufReader::new("+-<>,.[]".as_bytes());
        let parsed_codes = BFCode::<u8>::parse(source_codes).map_or_else(
            error_func,
            ok_func
        );

        assert_eq!(
            parsed_codes,
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

        // test the merge function
        let source_codes = BufReader::new("-<<<+++--><++>>>".as_bytes());
        let parsed_codes = BFCode::<u8>::parse(source_codes).map_or_else(
            error_func,
            ok_func
        );

        assert_eq!(
            parsed_codes,
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

        // test the brackets match function
        let source_codes = BufReader::new("[[][[[][[]][]]]]".as_bytes());
        let parsed_codes = BFCode::<u8>::parse(source_codes).map_or_else(
            error_func,
            ok_func
        );

        assert_eq!(
            parsed_codes,
            vec![
                Jz(15),   // 0
                Jz(2),    // 1
                Jnz(1),   // 2
                Jz(14),   // 3
                Jz(13),   // 4
                Jz(6),    // 5
                Jnz(5),   // 6
                Jz(10),   // 7
                Jz(9),    // 8
                Jnz(8),   // 9
                Jnz(7),   // 10
                Jz(12),   // 11
                Jnz(11),  // 12
                Jnz(4),   // 13
                Jnz(3),   // 14
                Jnz(0),   // 15
            ]
        );
    }
}

