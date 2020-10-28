use std::fmt::{Display, Formatter, Result};
use crate::syntax::ast::{Keyword, Span, Punctuator};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a token
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  /// The token data
  pub kind: TokenKind,
  /// Token position from source code
  pub span: Span,
}

impl Token {
  /// Create a new detail token from the token data, line number and column number
  #[inline]
  pub fn new(kind: TokenKind, span: Span) -> Self {
    Self { kind, span }
  }

  #[inline]
  pub fn kind(&self) -> &TokenKind {
    &self.kind
  }

  #[inline]
  pub fn span(&self) -> &Span {
    &self.span
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.kind)
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug)]
pub enum Numeric {
  /// A floating point number
  Rational(f64),

  /// An Integer
  Integer(i32),
}

impl From<f64> for Numeric {
  fn from(n: f64) -> Self {
    Self::Rational(n)
  }
}

impl From<i32> for Numeric {
  fn from(n: i32) -> Self {
    Self::Integer(n)
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
  /// 布尔值字符，`true` 和 `false`
  BooleanLiteral(bool),
  /// The end of the file, 文件结尾
  EOF,
  /// 变量、函数等标识符
  Identifier(Box<str>),
  /// 关键字
  Keyword(Keyword),
  /// NullLiteral
  NullLiteral,
  /// 数字
  NumericLiteral(Numeric),
  /// 标点
  Punctuator(Punctuator),
  /// String literal
  StringLiteral(Box<str>),
  TemplateLiteral(Box<str>),
  /// 一般表达式
  // RegularExpression(String),
  /// Indicates the end of a line (`\n`).
  LineTerminator,
  /// 注释
  Comment,
}

impl From<bool> for TokenKind {
  fn from(oth: bool) -> Self {
      Self::BooleanLiteral(oth)
  }
}

impl From<Keyword> for TokenKind {
  fn from(kw: Keyword) -> Self {
      Self::Keyword(kw)
  }
}

impl From<Punctuator> for TokenKind {
  fn from(punc: Punctuator) -> Self {
      Self::Punctuator(punc)
  }
}

impl From<Numeric> for TokenKind {
  fn from(num: Numeric) -> Self {
      Self::NumericLiteral(num)
  }
}

impl TokenKind {
  /// Creates a `BooleanLiteral` token kind.
  pub fn boolean_literal(lit: bool) -> Self {
      Self::BooleanLiteral(lit)
  }

  /// Creates an `EOF` token kind.
  pub fn eof() -> Self {
      Self::EOF
  }

  /// Creates an `Identifier` token type.
  pub fn identifier<I>(ident: I) -> Self
  where
      I: Into<Box<str>>,
  {
      Self::Identifier(ident.into())
  }

  /// Creates a `Keyword` token kind.
  pub fn keyword(keyword: Keyword) -> Self {
      Self::Keyword(keyword)
  }

  /// Creates a `NumericLiteral` token kind.
  pub fn numeric_literal<L>(lit: L) -> Self
  where
      L: Into<Numeric>,
  {
      Self::NumericLiteral(lit.into())
  }

  /// Creates a `Punctuator` token type.
  pub fn punctuator(punc: Punctuator) -> Self {
      Self::Punctuator(punc)
  }

  /// Creates a `StringLiteral` token type.
  pub fn string_literal<S>(lit: S) -> Self
  where
      S: Into<Box<str>>,
  {
      Self::StringLiteral(lit.into())
  }

  /// Creates a `TemplateLiteral` token type.
  pub fn template_literal<S>(lit: S) -> Self
  where
      S: Into<Box<str>>,
  {
      Self::TemplateLiteral(lit.into())
  }

  /// Creates a `RegularExpressionLiteral` token kind.
  // pub fn regular_expression_literal<B, R>(body: B, flags: R) -> Self
  // where
  //     B: Into<Box<str>>,
  //     R: Into<RegExpFlags>,
  // {
  //     Self::RegularExpressionLiteral(body.into(), flags.into())
  // }

  /// Creates a `LineTerminator` token kind.
  pub fn line_terminator() -> Self {
      Self::LineTerminator
  }

  /// Creates a 'Comment' token kind.
  pub fn comment() -> Self {
      Self::Comment
  }
}

impl Display for TokenKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match *self {
      TokenKind::BooleanLiteral(val) => write!(f, "{}", val),
      TokenKind::EOF => write!(f, "end of file"),
      TokenKind::Identifier(ref ident) => write!(f, "{}", ident),
      TokenKind::Keyword(word) => write!(f, "{}", word),
      TokenKind::NullLiteral => write!(f, "null"),
      TokenKind::NumericLiteral(Numeric::Rational(num)) => write!(f, "{}", num),
      TokenKind::NumericLiteral(Numeric::Integer(num)) => write!(f, "{}", num),
      TokenKind::Punctuator(ref punc) => write!(f, "{}", punc),
      TokenKind::StringLiteral(ref lit) => write!(f, "{}", lit),
      TokenKind::TemplateLiteral(ref lit) => write!(f, "{}", lit),
      // TokenKind::RegularExpression(reg) => write!(f, "{}", reg),
      TokenKind::LineTerminator => write!(f, "line terminator"),
      TokenKind::Comment => write!(f, "comment")
    }
  }
}
