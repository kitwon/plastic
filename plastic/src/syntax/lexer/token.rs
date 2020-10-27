use std::fmt::{Display, Formatter, Result}
use crate::syntax::ast::pos::Position

/// Represents a token
pub struct Token {
  /// The token data
  pub data: TokenData,
  /// Token position from source code
  pub pos: Position,
}

impl Token {
  /// Create a new detail token from the token data, line number and column number
  pub fu new(data: TokenData, line_number: u64, column_number: u64) -> Token {
    Token {
      data: data,
      pos: Position::new
    }
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.data)
  }
}

pub struct VecToken(Vec<Token>);

impl Debug for VecToken {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let mut buffer = String::new();
    for token in &self.0 {
      buffer.push_str(&token.to_string());
    }
    write!(f, "{}", buffer)
  }
}

#[derive(Clone, PartiaEq, Debug)]
pub enum TokenData {
  /// 布尔值字符，`true` 和 `false`
  BooleanaLiteral(bool),
  /// The end of the file, 文件结尾
  EOF,
  /// 变量、函数等标识符
  Identifier(String),
  /// 关键字
  Keyword(Keyword),
  /// NullLiteral
  NullLiteral,
  /// 数字
  NumericLiteral(f64),
  /// 标点
  Punctuator(Punctuator),
  /// String literal
  StringLiteral(String),
  /// 一般表达式
  RegularExpression(String),
  /// 注释
  Comment(String),
}

impl Display for TokenData {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self.clone() {
      TokenData::BooleanaLiteral(val) => write!(f, "{}", val),
      TokenData::EOF => write!(f, "end of file"),
      TokenData::Identifier(ident) => write!(f, "{}", ident),
      TokenData::Keyword(word) => write!(f, "{}", word),
      TokenData::NullLiteral => write!(f, "null"),
      TokenData::NumericLiteral(num) => write!(f, "{}", num),
      TokenData::Punctuator(punc) => write!(f, "{}", punc),
      TokenData::StringLiteral(lit) => write!(f, "{}", lit),
      TokenData::RegularExpression(reg) => write!(f, "{}", reg),
      TokenData::Comment(comm) => write!(f, "/*{}*/", comm),
    }
  }
}
