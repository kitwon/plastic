//! Module implementing the lexer cursor. This is used for managing the input byte stream.
//! 词法分析器的游标实现。处理输入的文档字节流。
//!
use crate::{syntax::ast::Position};
use std::io::{self, Bytes, Read, Error};

/// Cursor over the source code.
#[derive(Debug)]
pub(super) struct Cursor<R> {
  iter: InnerIter<R>,
  pos: Position,
}

impl<R> Cursor<R> {
  /// Gets the current position of the cursor in the sourece code.
  #[inline]
  pub(super) fn pos(&self) -> Position {
    self.pos
  }

  #[inline]
  pub(super) fn next_column(&mut self) {
    let current_line = self.pos.line_number();
    let next_column = self.pos.line_number() + 1;
    self.pos = Position::new(current_line, next_column)
  }

  #[inline]
  fn next_line(&mut self) {
    let next_line = self.pos.line_number() + 1;
    self.pos = Position::new(next_line, 1)
  }
}

impl<R> Cursor<R>
  where
    R: Read,
{
  /// Create a new lexer cursor
  #[inline]
  pub(super) fn new(inner: R) -> Self {
    Self {
      iter: InnerIter::new(inner.bytes()),
      pos: Position::new(1, 1),
    }
  }

  #[inline]
  pub(super) fn fill_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
    self.iter.fill_bytes(buf)
  }

  #[inline]
  pub(super) fn peek(&mut self) -> Result<Option<char>, Error> {
    self.iter.peek_char()
  }

  #[inline]
  pub(super) fn next_is(&mut self, peek: char) -> io::Result<bool> {
    Ok(match self.peek() ? {
      Some(next) if next == peek => {
        let _ = self.iter.next_char();
        true
      }
      _ => false,
    })
  }

  pub(super) fn next_is_pred<F>(&mut self, pred: &F) -> io::Result<bool>
    where
      F: Fn(char) -> bool
  {
    Ok(if let Some(peek) = self.peek() ? {
      pred(peek)
    } else {
      false
    })
  }

  pub(crate) fn next_char(&mut self) -> Result<Option<char>, Error> {
    let chr = self.iter.next_char()?;

    match chr {
      Some('\r') => {
        if self.peek()? == Some('\n') {
          let _ = self.iter.next_char();
        }
        self.next_line();
      },
      Some('\n') | Some('\u{2028}') | Some('\u{2029}') => self.next_line(),
      Some(_) => self.next_column(),
      None => {}
    }

    Ok(chr)
  }

  pub(super) fn take_until(&mut self, stop: char, buf: &mut String) -> io::Result<()> {
    loop {
      if self.next_is(stop) ? {
        return Ok(())
      } else if let Some(ch) = self.next_char() ? {
        buf.push(ch)
      } else {
        return Err(io::Error::new(
          io::ErrorKind::UnexpectedEof,
          format!("Unexpected end of file when looking for character {}", stop)
        ))
      }
    }
  }
}

#[derive(Debug)]
struct InnerIter<R> {
  iter: Bytes<R>,
  peeked_char: Option<Option<char>>,
}

impl<R> InnerIter<R> {
  fn new(iter: Bytes<R>) -> Self {
    Self {
      iter,
      peeked_char: None,
    }
  }
}

impl<R> InnerIter<R>
  where
    R: Read,
{
  fn next_char(&mut self) -> io::Result<Option<char>> {
    if let Some(v) = self.peeked_char.take() {
      return Ok(v)
    }

    let first_byte = match self.iter.next().transpose()? {
      Some(b) => b,
      None => return Ok(None)
    };

    let chr: char = if first_byte < 0x80 {
      first_byte.into()
    } else {
      let mut buf = [first_byte, 0u8, 0u8, 0u8];
      let num_bytes = if first_byte < 0xE0 {
        2
      } else if first_byte < 0xF0 {
        3
      } else {
        4
      };

      for b in buf.iter_mut().take(num_bytes).skip(1) {
        let next = match self.iter.next() {
          Some(Ok(b)) => b,
          Some(Err(e)) => return Err(e),
          None => {
            return Err(io::Error::new(
              io::ErrorKind::InvalidData,
              "stream did not contain valid UTF-8",
            ));
          }
        };

        *b = next;
      }

      if let Ok(s) = std::str::from_utf8(&buf) {
        if let Some(chr) = s.chars().next() {
          chr
        } else {
          return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "stream did not contain valid UTF-8"
          ));
        }
      } else {
        return Err(io::Error::new(
          io::ErrorKind::InvalidData,
          "stream did not contain valid UTF-8"
        ));
      }
    };

    Ok(Some(chr))
  }

  fn next_ascii(&mut self) -> io::Result<Option<u8>> {
    match self.next_char() {
      Ok(Some(chr)) if chr.is_ascii() => Ok(Some(chr as u8)),
      Ok(None) => Ok(None),
      _ => Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "non_ASCII byte found"
      ))
    }
  }

  fn fill_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
    for byte in buf.iter_mut() {
      *byte = self.next_ascii()?.ok_or_else(|| {
        io::Error::new(
          io::ErrorKind::UnexpectedEof,
          "unexpected EOF when filling buffer",
        )
      })?;
    }
    Ok(())
  }

  #[inline]
  pub(super) fn peek_char(&mut self) -> Result<Option<char>, Error> {
    if let Some(v) = self.peeked_char {
      Ok(v)
    } else {
      let chr = self.next_char()?;
      self.peeked_char = Some(chr);
      Ok(chr)
    }
  }
}
