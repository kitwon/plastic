use super::*;
use super::token::Numeric;
use crate::syntax::ast::Keyword;

fn span(start: (u32, u32), end: (u32, u32)) -> Span {
  Span::new(Position::new(start.0, start.1), Position::new(end.0, end.1))
}

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

#[test]
fn check_multi_line_comment() {
  let s = "var /*await \n break \n*/ x";
  let mut lexer = Lexer::new(s.as_bytes());

  let expected = [
    TokenKind::Keyword(Keyword::Var),
    TokenKind::LineTerminator,
    TokenKind::identifier("x"),
  ];

  expect_tokens(&mut lexer, &expected);
}

#[test]
fn check_string() {
  let s = "'aaa' \"bbb\"";
  let mut lexer = Lexer::new(s.as_bytes());

  let expected = [
    TokenKind::string_literal("aaa"),
    TokenKind::string_literal("bbb"),
  ];

  expect_tokens(&mut lexer, &expected);
}

#[test]
fn check_punctuators() {
  let s = "{ ( ) [ ] . ... ; , < > <= >= == != === !== \
    + - * % -- << >> >>> & | ^ ! ~ && || ? : \
    = += -= *= &= **= ++ ** <<= >>= >>>= &= |= ^= =>";

  let mut lexer = Lexer::new(s.as_bytes());

  let expected = [
    TokenKind::Punctuator(Punctuator::OpenBlock),
    TokenKind::Punctuator(Punctuator::OpenParen),
    TokenKind::Punctuator(Punctuator::CloseParen),
    TokenKind::Punctuator(Punctuator::OpenBracket),
    TokenKind::Punctuator(Punctuator::CloseBracket),
    TokenKind::Punctuator(Punctuator::Dot),
    TokenKind::Punctuator(Punctuator::Spread),
    TokenKind::Punctuator(Punctuator::Semicolon),
    TokenKind::Punctuator(Punctuator::Comma),
    TokenKind::Punctuator(Punctuator::LessThan),
    TokenKind::Punctuator(Punctuator::GreaterThan),
    TokenKind::Punctuator(Punctuator::LessThanOrEq),
    TokenKind::Punctuator(Punctuator::GreaterThanOrEq),
    TokenKind::Punctuator(Punctuator::Eq),
    TokenKind::Punctuator(Punctuator::NotEq),
    TokenKind::Punctuator(Punctuator::StrictEq),
    TokenKind::Punctuator(Punctuator::StrictNotEq),
    TokenKind::Punctuator(Punctuator::Add),
    TokenKind::Punctuator(Punctuator::Sub),
    TokenKind::Punctuator(Punctuator::Mul),
    TokenKind::Punctuator(Punctuator::Mod),
    TokenKind::Punctuator(Punctuator::Dec),
    TokenKind::Punctuator(Punctuator::LeftSh),
    TokenKind::Punctuator(Punctuator::RightSh),
    TokenKind::Punctuator(Punctuator::URightSh),
    TokenKind::Punctuator(Punctuator::And),
    TokenKind::Punctuator(Punctuator::Or),
    TokenKind::Punctuator(Punctuator::Xor),
    TokenKind::Punctuator(Punctuator::Not),
    TokenKind::Punctuator(Punctuator::Neg),
    TokenKind::Punctuator(Punctuator::BoolAnd),
    TokenKind::Punctuator(Punctuator::BoolOr),
    TokenKind::Punctuator(Punctuator::Question),
    TokenKind::Punctuator(Punctuator::Colon),
    TokenKind::Punctuator(Punctuator::Assign),
    TokenKind::Punctuator(Punctuator::AssignAdd),
    TokenKind::Punctuator(Punctuator::AssignSub),
    TokenKind::Punctuator(Punctuator::AssignMul),
    TokenKind::Punctuator(Punctuator::AssignAnd),
    TokenKind::Punctuator(Punctuator::AssignPow),
    TokenKind::Punctuator(Punctuator::Inc),
    TokenKind::Punctuator(Punctuator::Exp),
    TokenKind::Punctuator(Punctuator::AssignLeftSh),
    TokenKind::Punctuator(Punctuator::AssignRightSh),
    TokenKind::Punctuator(Punctuator::AssignURightSh),
    TokenKind::Punctuator(Punctuator::AssignAnd),
    TokenKind::Punctuator(Punctuator::AssignOr),
    TokenKind::Punctuator(Punctuator::AssignXor),
    TokenKind::Punctuator(Punctuator::Arrow),
  ];

  expect_tokens(&mut lexer, &expected);
}

#[test]
fn check_keywords() {
  // https://tc39.es/ecma262/#sec-keywords
  let s = "await break case catch class const continue debugger default delete \
           do else export extends finally for function if import in instanceof \
           new return super switch this throw try typeof var void while with yield";

  let mut lexer = Lexer::new(s.as_bytes());

  let expected = [
      TokenKind::Keyword(Keyword::Await),
      TokenKind::Keyword(Keyword::Break),
      TokenKind::Keyword(Keyword::Case),
      TokenKind::Keyword(Keyword::Catch),
      TokenKind::Keyword(Keyword::Class),
      TokenKind::Keyword(Keyword::Const),
      TokenKind::Keyword(Keyword::Continue),
      TokenKind::Keyword(Keyword::Debugger),
      TokenKind::Keyword(Keyword::Default),
      TokenKind::Keyword(Keyword::Delete),
      TokenKind::Keyword(Keyword::Do),
      TokenKind::Keyword(Keyword::Else),
      TokenKind::Keyword(Keyword::Export),
      TokenKind::Keyword(Keyword::Extends),
      TokenKind::Keyword(Keyword::Finally),
      TokenKind::Keyword(Keyword::For),
      TokenKind::Keyword(Keyword::Function),
      TokenKind::Keyword(Keyword::If),
      TokenKind::Keyword(Keyword::Import),
      TokenKind::Keyword(Keyword::In),
      TokenKind::Keyword(Keyword::InstanceOf),
      TokenKind::Keyword(Keyword::New),
      TokenKind::Keyword(Keyword::Return),
      TokenKind::Keyword(Keyword::Super),
      TokenKind::Keyword(Keyword::Switch),
      TokenKind::Keyword(Keyword::This),
      TokenKind::Keyword(Keyword::Throw),
      TokenKind::Keyword(Keyword::Try),
      TokenKind::Keyword(Keyword::TypeOf),
      TokenKind::Keyword(Keyword::Var),
      TokenKind::Keyword(Keyword::Void),
      TokenKind::Keyword(Keyword::While),
      TokenKind::Keyword(Keyword::With),
      TokenKind::Keyword(Keyword::Yield),
  ];

  expect_tokens(&mut lexer, &expected);
}

#[test]
fn check_variable_definition_tokens() {
  let s = "let a = 'hello';";
  let mut lexer = Lexer::new(s.as_bytes());

  let expected = [
    TokenKind::keyword(Keyword::Let),
    TokenKind::identifier("a"),
    TokenKind::Punctuator(Punctuator::Assign),
    TokenKind::string_literal("hello"),
    TokenKind::Punctuator(Punctuator::Semicolon),
  ];

  expect_tokens(&mut lexer, &expected);
}

#[test]
fn check_positions() {
  let s = r#"console.log("hello world"); // Test"#;
  let mut lexer = Lexer::new(s.as_bytes());

  // First column is 1 (none zero column)
  // console keyword
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 1), (1, 8)));

  // Dot punc
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 8), (1, 9)));

  // Log token should start on column 9.
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 9), (1, 12)));

  // Open parethesis token should start on column 12.
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 12), (1, 13)));

  // String literal should start on column 13
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 13), (1, 26)));

  // Close parethesis token should start on column 26.
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 26), (1, 27)));

  // Semi colon token should start on 27
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 27), (1, 28)));
}

#[test]
fn check_positions_codepoint() {
  let s = r#"console.log("hello world\u{{2764}}"); // Test"#;
  let mut lexer = Lexer::new(s.as_bytes());

  // First column is 1 (none zero column)
  // console keyword
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 1), (1, 8)));

  // Dot punc
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 8), (1, 9)));

  // Log token should start on column 9.
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 9), (1, 12)));

  // Open parethesis token should start on column 12.
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 12), (1, 13)));

  // String literal should start on column 31
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 13), (1, 34)));

  // Close parethesis token should start on column 26.
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 34), (1, 35)));

  // Semi colon token should start on 27
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 35), (1, 36)));
}

#[test]
fn check_line_numbers() {
  let s = "x\ny\n";
  let mut lexer = Lexer::new(s.as_bytes());

  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 1), (1, 2)));
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((1, 2), (2, 1)));
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((2, 1), (2, 2)));
  assert_eq!(lexer.next().unwrap().unwrap().span(), span((2, 2), (3, 1)));
}

#[test]
fn check_decrement_advances_lexer_2_places() {
  let s = "let a = b--;";
  let mut lexer = Lexer::new(s.as_bytes());

  for _ in 0..4 {
    lexer.next().unwrap();
  }

  assert_eq!(lexer.next().unwrap().unwrap().kind(), &TokenKind::Punctuator(Punctuator::Dec));
  assert_eq!(lexer.next().unwrap().unwrap().kind(), &TokenKind::Punctuator(Punctuator::Semicolon));
}

#[test]
fn numbers() {
    let mut lexer = Lexer::new(
        "1 2 0x34 056 7.89 42. 5e3 5e+3 5e-3 0b10 0O123 0999 1.0e1 1.0e-1 1.0E1 1E1 0.0 0.12 -32"
            .as_bytes(),
    );

    let expected = [
        TokenKind::numeric_literal(1),
        TokenKind::numeric_literal(2),
        TokenKind::numeric_literal(52),
        TokenKind::numeric_literal(46),
        TokenKind::numeric_literal(7.89),
        TokenKind::numeric_literal(42),
        TokenKind::numeric_literal(5000),
        TokenKind::numeric_literal(5000),
        TokenKind::numeric_literal(0.005),
        TokenKind::numeric_literal(2),
        TokenKind::numeric_literal(83),
        TokenKind::numeric_literal(999),
        TokenKind::numeric_literal(10),
        TokenKind::numeric_literal(0.1),
        TokenKind::numeric_literal(10),
        TokenKind::numeric_literal(10),
        TokenKind::numeric_literal(0),
        TokenKind::numeric_literal(0.12),
        TokenKind::Punctuator(Punctuator::Sub),
        TokenKind::numeric_literal(32),
    ];

    expect_tokens(&mut lexer, &expected);
}

#[test]
fn big_exp_numbers() {
    let mut lexer = Lexer::new(&b"1.0e25 1.0e36 9.0e50"[..]);

    let expected = [
        TokenKind::numeric_literal(10000000000000000000000000.0),
        TokenKind::numeric_literal(1000000000000000000000000000000000000.0),
        TokenKind::numeric_literal(900000000000000000000000000000000000000000000000000.0),
    ];

    expect_tokens(&mut lexer, &expected);
}

#[test]
#[ignore]
fn big_literal_numbers() {
    let mut lexer = Lexer::new(&b"10000000000000000000000000"[..]);

    let expected = [TokenKind::numeric_literal(10000000000000000000000000.0)];

    expect_tokens(&mut lexer, &expected);
}

#[test]
fn sigle_number_without_semicolon() {
  let mut lexer = Lexer::new(&b"1"[..]);
  if let Some(x) = lexer.next().unwrap() {
    assert_eq!(x.kind(), &TokenKind::numeric_literal(Numeric::Integer(1)))
  } else {
    panic!("Failed to lex 1 without semicolon")
  }
}

#[test]
fn take_while_pred_simple() {
  let mut cur = Cursor::new(&b"abcdefghijk"[..]);
  let mut buf: String = String::new();

  cur.take_while_pred(&mut buf, &|c| c == 'a' || c == 'b' || c == 'c').unwrap();
  assert_eq!(buf, "abc");
}

#[test]
fn take_while_immediate_stop() {
  let mut cur = Cursor::new(&b"abcdefghijk"[..]);
  let mut buf: String = String::new();

  cur.take_while_pred(&mut buf, &|c| c == 'd').unwrap();
  assert_eq!(buf, "");
}

#[test]
fn take_while_pred_entire_str() {
  let mut cur = Cursor::new(&b"abcdefghijk"[..]);

  let mut buf: String = String::new();

  cur.take_while_pred(&mut buf, &|c| c.is_alphabetic())
      .unwrap();

  assert_eq!(buf, "abcdefghijk");
}

#[test]
fn non_english_str() {
    let str = r#"'中文';"#;

    let mut lexer = Lexer::new(str.as_bytes());

    let expected = [
        TokenKind::StringLiteral("中文".into()),
        TokenKind::Punctuator(Punctuator::Semicolon),
    ];

    expect_tokens(&mut lexer, &expected);
}
