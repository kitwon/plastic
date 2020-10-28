use gc::{Finalize, Trace};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
/// A Javascript Constant
pub enum Const {
  /// A UTF-8 string, such as `"Hello, world"`
  String(Box<str>),
  // A regular expression, such as `/where('s| is) [wW]ally/`
  RegExp(String, bool, bool),
  // A 64-bit floating-point number, such as `3.1415`
  Num(f64),
  // A 32-bit integer, such as `42`
  Int(i32),
  // A boolean, which is either `true` or `false` and is used to check if criteria are met
  Bool(bool),
  // The `null` value, which represents a non-existant value
  Null,
  // The `undefined` value, which represents a field or index that doesn't exist
  Undefined,
}

impl From<&str> for Const {
  fn from(s: &str) -> Self {
    Self::String(s.to_owned().into_boxed_str())
  }
}

impl From<&String> for Const {
  fn from(s: &String) -> Self {
    Self::String(s.clone().into_boxed_str())
  }
}

impl From<Box<str>> for Const {
  fn from(s: Box<str>) -> Self {
    Self::String(s)
  }
}

impl From<String> for Const {
  fn from(s: String) -> Self {
    Self::String(s.into_boxed_str())
  }
}

impl From<f64> for Const {
  fn from(num: f64) -> Self {
    Self::Num(num)
  }
}

impl From<i32> for Const {
  fn from(num: i32) -> Self {
    Self::Int(num)
  }
}

impl From<bool> for Const {
  fn from(b: bool) -> Self {
    Self::Bool(b)
  }
}

impl Display for Const {
  fn fmt(&self, f: &mut Formatter) -> Result {
    return match *self {
      Self::String(ref st) => write!(f, "\"{}\"", st),
      Self::RegExp(ref reg, _, _) => write!(f, "~/{}/", reg),
      Self::Num(num) => write!(f, "{}", num),
      Self::Int(num) => write!(f, "{}", num),
      Self::Bool(v) => write!(f, "{}", v),
      Self::Null => write!(f, "null"),
      Self::Undefined => write!(f, "undefined"),
    };
  }
}
