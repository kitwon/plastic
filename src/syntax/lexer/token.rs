// This module implements all of the [Token]s use in Javascript

pub struct Token {
  // Token 类型，包括token所有可用值
  kind: TokenKind
  // Token 在源码中的位置
  span: Span
}

pub enum TokenKind {
  // Boolean 字符, `true` or `false`
  BooleanLiteral(bool),

  // The end of the file.
  EOF,

  Identifier(Box<str>),

  // A keyword.
  keyword(Keyword),

  // A `null` literal.
  NullLiteral
}
