use std::iter::Peekable

pub struct Lexer<'a> {
  pub tokens: Vec<Token>,
  line_number: u64,
  column_number: u64,
  buffer: Peekable<Chars<'a>>,
}
