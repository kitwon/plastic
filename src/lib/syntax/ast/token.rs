use crate::syntax::ast::pos::Position

/// Represents a token
pub struct Token {
  /// The token data
  pub data: TokenData,
  /// Token position from source code
  pub pos: Position,
}
