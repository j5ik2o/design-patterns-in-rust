use std::collections::VecDeque;
use std::fmt::Debug;

pub trait Command: Debug {
  fn execute(&self);
}

#[derive(Debug)]
pub struct MacroCommand<C: Command> {
  commands: VecDeque<C>,
}

impl<C: Command> Command for MacroCommand<C> {
  fn execute(&self) {
    for cmd in &self.commands {
      cmd.execute();
    }
  }
}

impl<C: Command> MacroCommand<C> {
  pub fn new() -> Self {
    Self {
      commands: VecDeque::new(),
    }
  }

  pub fn append(&mut self, cmd: C) {
    self.commands.push_back(cmd);
  }

  pub fn undo(&mut self) {
    if !self.commands.is_empty() {
      self.commands.pop_front();
    }
  }

  pub fn clear(&mut self) {
    self.commands.clear();
  }
}

#[derive(Debug)]
struct EchoCommand {
  msg: String,
}

impl EchoCommand {
  pub fn new(msg: &str) -> Self {
    Self { msg: msg.to_owned() }
  }

  fn run(&self) {
    println!("{}", self.msg)
  }
}

impl Command for EchoCommand {
  fn execute(&self) {
    self.run()
  }
}

#[derive(Debug)]
struct DoubleEchoCommand {
  msg: String,
}

impl DoubleEchoCommand {
  fn new(msg: &str) -> Self {
    Self { msg: msg.to_owned() }
  }
}

impl Command for DoubleEchoCommand {
  fn execute(&self) {
    println!("{}{}", self.msg, self.msg)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fmt::Formatter;

  #[test]
  fn test() {
    fn execute<T: Command>(cmd: &T) {
      cmd.execute()
    }

    let mut mc = MacroCommand::new();
    mc.append(EchoCommand::new("Hello"));
    // コンパイルエラーになる
    // mc.append(DoubleEchoCommand::new("Hello"));
    execute(&mc);
  }
}
