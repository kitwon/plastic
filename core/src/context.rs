use crate::{
  exec::Interpreter
};

#[derive(Debug)]
pub struct Context {
  executor: Interpreter,
}

impl Default for Context {
  fn default() -> Self {
    let executor = Interpreter::new();
    let context = Self {
      executor
    };

    context
  }
}

impl Context {
  pub fn new() -> Self {
    Default::default()
  }
}
