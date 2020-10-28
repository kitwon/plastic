
use crate::syntax::lexer::Token;
use std::io::Read;

#[derive(Debug)]
pub(super) enum SemicolonResult<'s> {
  Found(Option<&'s Token>),
  NotFound(&'s Token)
}

pub(super) struct Cursor<R> {
  buffered_lexer: BufferedLexer<R>
}

impl<R> Cursor<R>
  where
    R: Read,
{
  #[inline]
  pub(super) fn new(reader: R) -> Self {
    Self {
      buffered_lexer: Lexer::new(reader).into(),
    }
  }
}
