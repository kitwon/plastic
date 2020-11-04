#[derive(Debug, Eq, PartialEq)]
pub(crate) enum InterpreterState {
  Executing,
  Return,
  Break(Option<Box<str>>),
  Continue(Option<Box<str>>),
}

#[derive(Debug)]
pub struct Interpreter {
  state: InterpreterState,
}

impl Default for Interpreter {
  fn default() -> Self {
    Self::new()
  }
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      state: InterpreterState::Executing,
    }
  }

  #[inline]
  pub(crate) fn set_current_state(&mut self, new_state: InterpreterState) {
    self.state = new_state
  }

  #[inline]
  pub(crate) fn get_current_state(&self) -> &InterpreterState {
    &self.state
  }
}
