pub use self::error::{ParserError}
use std::io::Read;

trait TokenParser<R>: Sized
  where
    R: Read
{
  type Output;

  fn parse(self, cursor: &mut Cursor<R>>) -> Result<Self::Output, ParserError>
}
