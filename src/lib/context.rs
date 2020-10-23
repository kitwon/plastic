pub struct Context {
  /// Realm holds the global object and the environment
  /// TODO
  realm: &str,

  /// The current executor.
  /// TODO
  executor: &str,

  /// Symbol hash
  ///
  /// 在这使用u32
  symbol_count: u32,
  /// TODO
  console: &str,
  /// TODO
  well_known_symbols: &str,
}

impl Default for Context {
  fn default() -> Self {
    let mut context = Self {
      "realm",
      "exector",
      0,
      "console",
      "well_know_symbols"
    }

    context
  }
}

impl Context {
  /// Throw a `SyntaxError` with specified message.
  pub fn throw_syntax_error<M>(&mut self, message: M) -> Result<Value>
    where
      M: Into<String>,
  {
    Err(message)
  }

  /// Evaluatues the give code.
  ///
  pub fn eval(&mut self, src: &str) -> Result<Value> {
    let parsing_result = Parser::new(src.as_bytes()())
      .parse_all()
      .map_err(|e| e.to_string())

    let execution_result = match parsing_result {
      Ok(statement_list) => statement_list.run(self),
      Err(e) => self.throw_syntax_error(e),
    }

    execution_result
  }
}
