use super::*;
use crate::syntax::ast::Keyword;

fn expect_tokens<R>(lexer: &mut Lexer<R>, expected: &[TokenKind])
  where
    R: Read,
{
  for expect in expected.iter() {
    assert_eq!(&lexer.next().unwrap().unwrap().kind(), &expect);
  }

  assert!(
    lexer.next().unwrap().is_none(),
    "Unexpected extra token lexed at end of input"
  );
}

#[test]
fn check_single_line_comment() {
  let s = "var \n//This is a comment\ntrue";
  let mut lexer = Lexer::new(s.as_bytes());

  let expected = [
    TokenKind::Keyword(Keyword::Var),
    TokenKind::LineTerminator,
    TokenKind::LineTerminator,
    TokenKind::BooleanLiteral(true),
  ];

  expect_tokens(&mut lexer, &expected);
}
