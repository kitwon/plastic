mod comment;
mod cursor;
mod error;
mod string;
mod operator;
pub mod token;

#[cfg(test)]
mod tests;

use self::{
  cursor::Cursor,
  comment::{SingleLineComment, MultiLineComment},
  string::{StringLiteral},
  operator::Operator,
};
use crate::syntax::ast::{Punctuator, Span};
pub use crate::{syntax::ast::Position};
pub use token::{Token, TokenKind};
pub use error::Error;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputElement {
  Div,
  RegExp,
  TemplateTail
}

impl Default for InputElement {
  fn default() -> Self {
    InputElement::RegExp
  }
}

trait Tokenizer<R> {
  fn lex(&mut self, cursor: &mut Cursor<R>, start_pos: Position) -> Result<Token, Error>
    where
      R: Read;
}

#[derive(Debug)]
pub struct Lexer<R> {
  cursor: Cursor<R>,
  goal_symbol: InputElement,
}

// TODO: Strict mode
//
impl<R> Lexer<R> {
  /// Check if a charater is whitespace as per ECMAScript standers
  ///
  /// [More information](https://tc39.es/ecma262/#table-32)
  fn is_whitespace(ch: char) -> bool {
    matches!(
      ch,
      '\u{0020}' | '\u{0009}' | '\u{000B}' | '\u{000C}' | '\u{00A0}' | '\u{FEFF}' |
      // Unicode Space_Seperator category (minus \u{0020} and \u{00A0} which are allready stated above)
      '\u{1680}' | '\u{2000}'..='\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}'
    )
  }

  #[inline]
  pub(crate) fn set_goal(&mut self, elm: InputElement) {
    self.goal_symbol = elm
  }

  #[inline]
  pub(crate) fn get_goal(&mut self) -> InputElement {
    self.goal_symbol
  }

  #[inline]
  pub fn new(reader: R) -> Self
    where R: Read
  {
    Self {
      cursor: Cursor::new(reader),
      goal_symbol: Default::default(),
    }
  }

  pub(crate) fn lex_slash_token(&mut self, start: Position) -> Result<Token, Error>
    where R: Read,
  {
    if let Some(c) = self.cursor.peek() ? {
      match c {
        '/' => {
          self.cursor.next_char()?.expect("/ token vanished");
          SingleLineComment.lex(&mut self.cursor, start)
        }
        '*' => {
          self.cursor.next_char()?.expect("* toekn vanished");
          MultiLineComment.lex(&mut self.cursor, start)
        }
        ch => {
          match self.get_goal() {
            InputElement::Div | InputElement::TemplateTail => {
              if ch == '=' {
                self.cursor.next_char()?.expect("= token vanished");
                Ok(Token::new(
                  Punctuator::AssignDiv.into(),
                  Span::new(start, self.cursor.pos()),
                ))
              } else {
                Ok(Token::new(
                  Punctuator::Div.into(),
                  Span::new(start, self.cursor.pos())
                ))
              }
            }
            // TODO: Match Regexp
          }
        }
      }
    } else {
      Err(Error::syntax(
        "Expecting Token /,*/= or regex",
        start,
      ))
    }
  }

  /// Retrieves the next token from the lexer
  pub fn next(&mut self) -> Result<Option<Token>, Error>
    where
      R: Read,
  {
    let (start, next_chr) = loop {
      let start = self.cursor.pos();
      if let Some(next_chr) = self.cursor.next_char()? {
        if !Self::is_whitespace(next_chr) {
          break (start, next_chr);
        }
      } else {
        return Ok(None);
      }
    };

    // TODO
    // Matched and Parser token
    let token = match next_chr {
      '\r' | '\n' | '\u{2028}' | '\u{2029}' => Ok(Token::new(
        TokenKind::LineTerminator,
        Span::new(start, self.cursor.pos()),
      )),
      '"' | '\'' => StringLiteral::new(next_chr).lex(&mut self.cursor, start),
      ';' => Ok(Token::new(
        Punctuator::Semicolon.into(),
        Span::new(start, self.cursor.pos()),
      )),
      ':' => Ok(Token::new(
        Punctuator::Colon.into(),
        Span::new(start, self.cursor.pos()),
      )),
      '(' => Ok(Token::new(
        Punctuator::OpenParen.into(),
        Span::new(start, self.cursor.pos())
      )),
      ')' => Ok(Token::new(
        Punctuator::CloseParen.into(),
        Span::new(start, self.cursor.pos())
      )),
      ',' => Ok(Token::new(
        Punctuator::Comma.into(),
        Span::new(start, self.cursor.pos())
      )),
      '{' => Ok(Token::new(
        Punctuator::OpenBlock.into(),
        Span::new(start, self.cursor.pos())
      )),
      '}' => Ok(Token::new(
        Punctuator::CloseBlock.into(),
        Span::new(start, self.cursor.pos())
      )),
      '[' => Ok(Token::new(
        Punctuator::OpenBracket.into(),
        Span::new(start, self.cursor.pos())
      )),
      ']' => Ok(Token::new(
        Punctuator::CloseBracket.into(),
        Span::new(start, self.cursor.pos())
      )),
      '?' => Ok(Token::new(
        Punctuator::Question.into(),
        Span::new(start, self.cursor.pos())
      )),
      '?' => Ok(Token::new(
        Punctuator::Question.into(),
        Span::new(start, self.cursor.pos())
      )),
      '=' | '*' | '+' | '-' | '%' | '|' | '&' | '^' | '<' | '>' | '!' | '~' => {
        Operator::new(next_chr).lex(&mut self.cursor, start)
      },
      _ => {
        let details = format!(
          "unexpected '{}' at line {}, column {}",
          next_chr,
          start.line_number(),
          start.column_number()
        );
        Err(Error::syntax(details, start))
      }
    }?;

    if token.kind() == &TokenKind::Comment {
      self.next()
    } else {
      Ok(Some(token))
    }
  }
}
