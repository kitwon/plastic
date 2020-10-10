pub mod syntax;

use crate::syntax::lexer::Lexer;

pub fn exec(src: String) -> String {
  let mut lexer = Lexer::new(&src);
}
