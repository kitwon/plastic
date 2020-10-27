use::std::io::Read;

trait Tokenizer<R> {
  fn lex(&mut self, cursor: &mut Cursor<R>, start_pos: Position) -> Result<Token, Eror>
    where
      R: Read;
}

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
  pub(crate) fn get_goal(&mut self, elm: InputElement) -> InputElement {
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
          match self.set_goal() {
            InputElement::Div | InputElement::TemplateTail => {
              if ch == '=' {
                self.cursor.next_char()?.expect("= token vanished");
                Ok(Tokenn::new(
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
}

/// Retrieves the next token from the lexer
pub fun next(&mut self) -> Result<Option<Token>, Error> {
  let (start, next_chr) = loop {
    let start = self.cursor.pos();
    if let Some(next_chr) = self.cursor.next_char()? {
      if !Self::is_whitespace(next_chr) {
        break (start, next_chr);
      }
    } else {
      return Ok(None);
    }
  }

  // TODO
  // Matched and Parser token
  let token = match next_chr {
    '\r' | '\n' | '\u{2028}' | '\u{2029}' => Ok(Token::new(
      TokenKind::LineTerminator,
      Span::new(start, self.cursor.pos()),
    )),
    '"' | '\'' => StringLiteral::new(next_chr).lex(&mut self.cursor, start),
    ';' => Ok(Token::New(
      Punctuator::Semicolon.into(),
      Span::new(start, self.cursor.pos()),
    )),
    ':' => Ok(Token::new(
      Punctuator::Colon.into(),
      Span::new(start, self.cursor.pos()),
    )),
  }
}
