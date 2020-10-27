use std::{cmp::Ordering, num::NonZeroU32, fmt};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A Position in the Javascript source Code.
///
/// Stores both the column number and line number.
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
  /// Line number.
  line_number: NonZeroU32,
  /// Column number.
  column_number: NonZeroU32,
}

impl Position {
  #[inline]
  pub fn new(line_number: u32, column_number: u32) -> Self {
    Self {
      line_number: NonZeroU32::new(line_number).expect("line number cannot be 0"),
      column_number: NonZeroU32::new(column_number).expect("column number cannot be 0"),
    }
  }

  #[inline]
  pub fn line_number(self) -> u32 {
    self.line_number.get()
  }

  #[inline]
  pub fn column_number(self) -> u32 {
    self.column_number.get()
  }
}

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}:{}", self.line_number, self.column_number)
  }
}

/// A span in the Javascript code.
///
/// Stroes a start position and an end positon.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
  start: Position,
  end: Position,
}

impl Span {
  #[inline]
  pub fn new(start: Position, end: Position) -> Self {
    assert!(start <= end, "a span cannot start after its end");

    Self { start, end }
  }

  #[inline]
  pub fn start(self) -> Position {
    self.start
  }

  #[inline]
  pub fn end(self) -> Position {
    self.end
  }

  #[inline]
  pub fn contains<S>(self, other: S) -> bool
  where
    S: Into<Self>,
  {
    let other = other.into();
    self.start <= other.start && self.end >= other.end
  }
}

impl From<Position> for Span {
  fn from(pos: Position) -> Self {
    Self {
      start: pos,
      end: pos,
    }
  }
}

impl PartialOrd for Span {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self == other {
      Some(Ordering::Equal)
    } else if self.end < other.start {
      Some(Ordering::Less)
    } else if self.start > other.end {
      Some(Ordering::Greater)
    } else {
      None
    }
  }
}

impl fmt::Display for Span {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}..{}]", self.start, self.end)
  }
}

#[cfg(test)]
mod tests {
  use super::{Position, Span};

  #[test]
  #[should_panic]
  fn invalid_position_column() {
    Position::new(10, 0);
  }

  #[test]
  #[should_panic]
  fn invalid_position_line() {
    Position::new(0, 10);
  }

  #[test]
  fn position_equality() {
    assert_eq!(Position::new(10, 50), Position::new(10, 50));
    assert_ne!(Position::new(10, 50), Position::new(10, 51));
    assert_ne!(Position::new(10, 50), Position::new(11, 50));
    assert_ne!(Position::new(10, 50), Position::new(11, 51));
  }

  #[test]
  fn position_order() {
    assert!(Position::new(10, 50) < Position::new(10, 51));
    assert!(Position::new(9, 50) < Position::new(10, 51));
    assert!(Position::new(10, 50) < Position::new(11, 50));
    assert!(Position::new(10, 50) < Position::new(11, 51));

    assert!(Position::new(10, 51) > Position::new(10, 50));
    assert!(Position::new(10, 50) > Position::new(9, 50));
    assert!(Position::new(11, 51) > Position::new(10, 50));
    assert!(Position::new(11, 49) > Position::new(10, 50));
  }

  #[test]
  fn position_getters() {
    let pos = Position::new(10, 50);

    assert_eq!(pos.line_number(), 10);
    assert_eq!(pos.column_number(), 50);
  }

  #[test]
  fn position_to_string() {
    let pos = Position::new(10, 50);

    assert_eq!("10:50", pos.to_string());
    assert_eq!("10:50", format!("{}", pos));
  }

  #[test]
  #[should_panic]
  fn invalid_span() {
    let a = Position::new(10, 30);
    let b = Position::new(10, 50);
    Span::new(b, a);
  }

  #[test]
  fn span_creation() {
    let a = Position::new(10, 30);
    let b = Position::new(10, 50);

    let _ = Span::new(a, b);
    let _ = Span::new(a, b);
    let _ = Span::from(a);
  }

  #[test]
  fn span_equality() {
    let a = Position::new(10, 50);
    let b = Position::new(10, 52);
    let c = Position::new(11, 20);

    let span_ab = Span::new(a, b);
    let span_ab_2 = Span::new(a, b);
    let span_ac = Span::new(a, c);
    let span_bc = Span::new(b, c);

    assert_eq!(span_ab, span_ab_2);
    assert_ne!(span_ab, span_ac);
    assert_ne!(span_ab, span_bc);
    assert_ne!(span_bc, span_ac);

    let span_a = Span::from(a);
    let span_aa = Span::new(a, a);

    assert_eq!(span_a, span_aa);
  }

  #[test]
  fn span_to_string() {
    let a = Position::new(10, 50);
    let b = Position::new(11, 51);
    let span = Span::new(a, b);

    assert_eq!("[10:50..11:51]", span.to_string());
    assert_eq!("[10:50..11:51]", format!("{}", span));
  }

  #[test]
  fn span_ordering() {
    let a = Position::new(10, 50);
    let b = Position::new(10, 52);
    let c = Position::new(11, 20);
    let d = Position::new(12, 5);

    let span_ab = Span::new(a, b);
    let span_cd = Span::new(c, d);

    assert!(span_ab < span_cd);
    assert!(span_cd > span_ab);
  }
}

