// use crate::syntax::lexer::Lexer;

pub mod syntax;
pub mod exec;
pub mod context;

#[cfg(test)]
pub(crate) fn exec(src: &str) -> String {
  // let mut lexer = Lexer::new(&src);
  String::from(src)
}
