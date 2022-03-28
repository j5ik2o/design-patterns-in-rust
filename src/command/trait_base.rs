use std::collections::VecDeque;
use std::fmt::Debug;

pub trait Command: Debug {
  fn execute(&self);
}

#[derive(Debug)]
pub struct MacroCommand {
  commands: VecDeque<Box<dyn Command>>,
}

impl Command for MacroCommand {
  fn execute(&self) {
    for cmd in &self.commands {
      cmd.execute();
    }
  }
}

impl MacroCommand {
  pub fn new() -> Self {
    Self {
      commands: VecDeque::new(),
    }
  }

  pub fn append(&mut self, cmd: Box<dyn Command>) {
    self.commands.push_back(cmd);
  }

  pub fn undo(&mut self) {
    if !self.commands.is_empty() {
      self.commands.pop_back();
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
  fn new(msg: &str) -> Self {
    Self { msg: msg.to_owned() }
  }
}

impl Command for EchoCommand {
  fn execute(&self) {
    println!("{}", self.msg)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fmt::Formatter;

  #[test]
  fn test() {
    let mut mc = MacroCommand::new();
    mc.append(Box::new(EchoCommand::new("Hello")));
    mc.execute();
  }
}
