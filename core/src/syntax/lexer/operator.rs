use super::{Tokenizer, Cursor, Error, Token};
use std::io::Read;
use crate::syntax::{
  ast::{Position, Span, Punctuator}
};

macro_rules! vop {
  ($cursor: ident, $assign_op: expr, $op: expr) => {
    match $cursor.peek() ? {
      None => Err(Error::syntax("abrupt end - could not preview next value as part of the operator", $cursor.pos())),
      Some('=') => {
        $cursor.next_char()?.expect("= token vanished");
        $cursor.next_column();
        $assign_op
      },
      Some(_) => $op,
    }
  };
  ($cursor: ident, $assign_op: expr, $op: expr, {$($case: pat => $block: expr), +}) => ({
    match $cursor.peek()? {
      None => Err(Error::syntax("abrupt end - could not preview next value as part of the operator", $cursor.pos())),
      Some('=') => {
        $cursor.next_char()?.expect("= token vanished");
        $cursor.next_column();
        $assign_op
      },
      $($case => {
        $cursor.next_char()?.expect("token vanished");
        $cursor.next_column();
        $block
      })+,
      _ => $op
    }
  })
}

macro_rules! op {
  ($crusor: ident, $start_pos: expr, $assign_op: expr, $op: expr) => {
    Ok(Token::new(
      vop!($cursor, $assign_op, $op)?.into(),
      Span::new($start_pos, $cursor.pos()),
    ))
  };
  ($cursor: ident, $start_pos: expr, $assign_op: expr, $op: expr, {$($case: pat => $block: expr), +}) => ({
    let punc: Punctuator = vop!($cursor, $assign_op, $op, {$($case => $block),+})?;
    Ok(Token::new(
      punc.into(),
      Span::new($start_pos, $cursor.pos()),
    ))
  })
}
/// Operator lexing.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#sec-ecmascript-language-expressions
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operator
#[derive(Debug, Clone, Copy)]
pub(super) struct Operator {
  init: char,
}

impl Operator {
  pub(super) fn new(init: char) -> Self {
    Self { init }
  }
}

impl<R> Tokenizer<R> for Operator {
  fn lex(&mut self, cursor: &mut Cursor<R>, start_pos: Position) -> Result<Token, Error>
    where
      R: Read,
  {
    match self.init {
      '*' => op!(cursor, start_pos, Ok(Punctuator::AssignMul), Ok(Punctuator::Mul), {
        Some('*') => vop!(cursor, Ok(Punctuator::AssignPow), Ok(Punctuator::Exp))
      })
    }
  }
}
