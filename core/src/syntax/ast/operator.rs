use gc::{Trace, Finalize, unsafe_empty_trace};
use std::fmt::{Display, Formatter, Result};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Finalize, Debug, PartialEq)]
pub enum NumOp {
  /// `a + b`
  Add,
  /// `a - b`
  Sub,
  /// `a / b`
  Div,
  /// `a * b`
  Mul,
  /// `a ** b`
  Exp,
  /// `a % b`
  Mod,
}

impl Display for NumOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "{}",
      match *self {
        Self::Add => "+",
        Self::Sub => "-",
        Self::Div => "/",
        Self::Mul => "*",
        Self::Exp => "**",
        Self::Mod => "%",
      }
    )
  }
}

unsafe impl Trace for NumOp {
  unsafe_empty_trace!();
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Finalize, Debug, PartialEq)]
pub enum CompOp {
  /// `a == b`
  Equal,
  /// `a != b`
  NotEqual,
  /// `a === b`
  StrictEqual,
  /// `a !== b`
  StrictNotEqual,
  /// `a > b`
  GreaterThan,
  /// `a >= b`
  GreaterThanOrEqual,
  /// `a < b`
  LessThan,
  /// `a <= b`
  LessThanOrEqual,
  /// `a in b`
  In,
  /// `a instanceOf b`
  InstanceOf,
}

impl Display for CompOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "{}",
      match *self {
        Self::Equal => "==",
        Self::NotEqual => "!=",
        Self::StrictEqual => "===",
        Self::StrictNotEqual => "!==",
        Self::GreaterThan => ">",
        Self::GreaterThanOrEqual => ">=",
        Self::LessThan => "<=",
        Self::LessThanOrEqual => "<=",
        Self::In => "in",
        Self::InstanceOf => "instanceOf"
      }
    )
  }
}

unsafe impl Trace for CompOp {
  unsafe_empty_trace!();
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Finalize, Debug, PartialEq)]
pub enum LogOp {
  /// `a && b`
  And,
  /// `a || b`
  Or,
}

impl Display for LogOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "{}",
      match &self {
        Self::And => "&&",
        Self::Or => "||",
      }
    )
  }
}

unsafe impl Trace for LogOp {
  unsafe_empty_trace!();
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Finalize, Debug, PartialEq)]
pub enum BinOp {
  Num(NumOp),
  Comp(CompOp),
  Log(LogOp),
  Comma,
}

impl From<NumOp> for BinOp {
  fn from(op: NumOp) -> Self {
    Self::Num(op)
  }
}

impl From<CompOp> for BinOp {
  fn from(op: CompOp) -> Self {
    Self::Comp(op)
  }
}

impl From<LogOp> for BinOp {
  fn from(op: LogOp) -> Self {
    Self::Log(op)
  }
}

impl Display for BinOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "{}",
      match *self {
        Self::Num(ref op) => op.to_string(),
        Self::Comp(ref op) => op.to_string(),
        Self::Log(ref op) => op.to_string(),
        Self::Comma => ",".to_string(),
      }
    )
  }
}

unsafe impl Trace for BinOp {
  unsafe_empty_trace!();
}
