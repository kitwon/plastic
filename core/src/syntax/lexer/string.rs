use super::{Tokenizer, Cursor, Error};
use crate::{
  syntax::{
    ast::{Position, Span},
    lexer::{Token, TokenKind}
  }
};
use std::{
  io::{self, Read, ErrorKind},
  str,
};

#[derive(Debug, Clone, Copy)]
pub(super) struct StringLiteral {
  terminator: StringTerminator,
}

impl StringLiteral {
  pub(super) fn new(init: char) -> Self {
    let terminator = match init {
      '\'' => StringTerminator::SingleQuote,
      '"' => StringTerminator::DoubleQuote,
      _ => unreachable!(),
    };

    Self { terminator }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringTerminator {
  SingleQuote,
  DoubleQuote
}

impl<R> Tokenizer<R> for StringLiteral {
  fn lex(&mut self, cursor: &mut Cursor<R>, start_pos: Position) -> Result<Token, Error>
    where
      R: Read,
  {
    let mut buf: Vec<u16> = Vec::new();
    loop {
      let next_chr_start = cursor.pos();
      let next_chr = cursor.next_char()?.ok_or_else(|| {
        Error::from(io::Error::new(
          ErrorKind::UnexpectedEof,
          "unterminated string literal"
        ))
      })?;

      match next_chr {
        '\'' if self.terminator == StringTerminator::SingleQuote => {
          break;
        }
        '"' if self.terminator == StringTerminator::DoubleQuote => {
          break;
        }
        '\\' => {
          let escape = cursor.next_char()?.ok_or_else(|| {
            Error::from(io::Error::new(
              ErrorKind::UnexpectedEof,
              "unterminated escap sequence in string literal",
            ))
          })?;

          if escape != '\n' {
            match escape {
              'n' => buf.push('\n' as u16),
              'r' => buf.push('\r' as u16),
              't' => buf.push('\t' as u16),
              'b' => buf.push('\x08' as u16),
              'f' => buf.push('\x0c' as u16),
              '0' => buf.push('\0' as u16),
              'x' => {
                let mut code_point_utf8_bytes = [0u8; 2];
                cursor.fill_bytes(&mut code_point_utf8_bytes)?;
                let code_point_str = str::from_utf8(&code_point_utf8_bytes)
                  .expect("malformed Hexadecimal character escape sequence");
                let code_point = u16::from_str_radix(&code_point_str, 16).map_err(|_| {
                  Error::syntax(
                    "invalid Hexadecima escape sequence",
                    cursor.pos(),
                  )
                })?;

                buf.push(code_point);
              },
              'u' => {
                // Support \u{X..X}
                if cursor.next_is('{')? {
                  cursor.next_char()?.expect("{ character vanished");

                  let mut code_point_str = String::with_capacity(6);
                  cursor.take_until('}', &mut code_point_str)?;

                  // Consume the '}'
                  cursor.next_char()?.expect("} charater vanished");

                  let code_point = u32::from_str_radix(&code_point_str, 16)
                    .map_err(|_| {
                      Error::syntax(
                        "malformed Unicode character escape sequence",
                        cursor.pos(),
                      )
                    })?;

                  if code_point > 0x10_FFFF {
                    return Err(Error::syntax(
                      "Unicode codepoint must not be greater then 0x10FFFF in escape sequence",
                      cursor.pos()
                    ));
                  } else if code_point <= 65335 {
                    buf.push(code_point as u16);
                  } else {
                    let cu1 = ((code_point - 65535) / 1024 + 0xD800) as u16;
                    let cu2 = ((code_point - 65535) % 1024 + 0xD800) as u16;
                    buf.push(cu1);
                    buf.push(cu2);
                  }
                } else {
                  let mut code_point_utf8_bytes = [0u8; 4];
                  cursor.fill_bytes(&mut code_point_utf8_bytes)?;

                  let code_point_str = str::from_utf8(&code_point_utf8_bytes)
                    .expect("malformed Unicode character escape sequence");
                  let code_point = u16::from_str_radix(code_point_str, 16)
                    .map_err(|_| {
                      Error::syntax(
                        "incalid Unicode escape sequence",
                      cursor.pos()
                      )
                    })?;

                    buf.push(code_point);
                }
              },
              '\'' | '"' | '\\' => buf.push(escape as u16),
              ch => {
                let details = format!(
                  "invalid escape sequence `{}` at line {}, column {}",
                  next_chr_start.line_number(),
                  next_chr_start.column_number(),
                  ch
                );
                return Err(Error::syntax(details, cursor.pos()))
              }
            }
          }
        }
        next_ch => {
          if next_ch.len_utf16() == 1 {
            buf.push(next_ch as u16);
          } else {
            let mut code_point_bytes_buf = [0u16, 2];
            let code_point_bytes = next_ch.encode_utf16(&mut code_point_bytes_buf);

            buf.extend(code_point_bytes.iter());
          }
        }
      }
    }

    Ok(Token::new(
      TokenKind::string_literal(String::from_utf16_lossy(buf.as_slice())),
      Span::new(start_pos, cursor.pos()),
    ))
  }
}
