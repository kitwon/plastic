use super::{Cursor, Tokenizer, Error};
use crate::{
  syntax::{
    ast::{Position, Keyword, Span},
    lexer::{Token, TokenKind},
  }
};
use std::io::Read;

const STRICT_FORBIDDEN_IDENTIFIERS: [&str; 11] = [
    "eval",
    "arguments",
    "implements",
    "interface",
    "let",
    "package",
    "private",
    "protected",
    "public",
    "static",
    "yield",
];

/// Identifier lexing.
///
pub(super) struct Identifier {
  init: char,
}

impl Identifier {
  pub(super) fn new(init: char) -> Self {
    Self { init }
  }
}

impl<R> Tokenizer<R> for Identifier {
  fn lex(&mut self, cursor: &mut Cursor<R>, start_pos: Position) -> Result<Token, Error>
    where
      R: Read,
  {
    let mut buf = self.init.to_string();

    cursor.take_while_pred(&mut buf, &|c: char| {
      c.is_alphabetic() || c.is_digit(10) || c == '_'
    })?;

    let tk = match buf.as_str() {
      "true" => TokenKind::BooleanLiteral(true),
      "false" => TokenKind::BooleanLiteral(false),
      "null" => TokenKind::NullLiteral,
      slice => {
        if let Ok(keyword) = slice.parse() {
          if cursor.strict_mode() && keyword == Keyword::With {
            return Err(Error::Syntax(
              "using 'with' statement not allow in strict mode".into(),
              start_pos
            ))
          }
          TokenKind::Keyword(keyword)
        } else {
          if cursor.strict_mode() && STRICT_FORBIDDEN_IDENTIFIERS.contains(&slice) {
            return Err(Error::Syntax(
              format!(
                "using feature reserved keywod '{}' not allowed in strict mode",
                slice
              ).into(),
              start_pos
            ));
          }
          TokenKind::identifier(slice)
        }
      }
    };

    Ok(Token::new(tk, Span::new(start_pos, cursor.pos())))
  }
}
