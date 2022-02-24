use std::collections::VecDeque;

pub trait Command {
  fn execute(&self);
}

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
      self.commands.pop_front();
    }
  }

  pub fn clear(&mut self) {
    self.commands.clear();
  }
}

#[cfg(test)]
mod test {
  use super::*;

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

  #[test]
  fn test() {
    let mut mc = MacroCommand::new();
    mc.append(Box::new(EchoCommand::new("Hello")));
    mc.execute();
  }
}
