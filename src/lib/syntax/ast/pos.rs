pub struct Position {
  // Column number
  pub column_number: u64,
  // Line number
  pub line_number: u64,
}

impl Position {
  /// Create a new positio for Tokens
  ///
  /// * `line_number` - The line number the token start at
  /// * `column_number` - The column number the token start at
  pub fn new(line_number: u64, column_number: u64) -> Position {
    Position {
      line_number: line_number,
      column_number: column_number,
    }
  }
}
