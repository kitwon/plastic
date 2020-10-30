use super::{Cursor, Error, Tokenizer};
use crate::{
  syntax::{
    ast::{Position, Span},
    lexer::{Token, TokenKind},
  }
};
use std::io::Read;

pub(super) struct SingleLineComment;

/// Lexes a single line comment.
///
impl<R> Tokenizer<R> for SingleLineComment {
  fn lex(&mut self, cursor: &mut Cursor<R>, start_pos: Position) -> Result<Token, Error>
    where
      R: Read,
  {
    while let Some(ch) = cursor.peek()? {
      if ch == '\n' {
        break;
      } else {
        cursor.next_char()?.expect("Comment character vansihed");
      }
    }
    Ok(Token::new(
      TokenKind::Comment,
      Span::new(start_pos, cursor.pos())
    ))
  }
}

/// Lexes a block comment.
///
pub(super) struct MultiLineComment;

impl<R> Tokenizer<R> for MultiLineComment{
  fn lex(&mut self, cursor: &mut Cursor<R>, start_pos: Position) -> Result<Token, Error>
    where
      R: Read,
  {
    let mut new_line = false;
    loop {
      if let Some(ch) = cursor.next_char()? {
        if ch == '*' && cursor.next_is('/') ? {
          break;
        } else if ch == '\n' {
          new_line = true
        }
      } else {
        return Err(Error::syntax(
          "Unterminated multiline comment",
          cursor.pos(),
        ))
      }
    }

    Ok(Token::new(
      if new_line {
        TokenKind::LineTerminator
      } else {
        TokenKind::Comment
      },
      Span::new(start_pos, cursor.pos())
    ))
  }
}
