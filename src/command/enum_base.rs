use std::collections::VecDeque;

pub enum Command {
  Echo(String),
  Double(String),
  Macro(MacroCommand),
}

pub struct MacroCommand {
  commands: VecDeque<Command>,
}

impl MacroCommand {
  pub fn append(&mut self, cmd: Command) {
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

impl Command {
  pub fn of_echo(s: &str) -> Self {
    Command::Echo(s.to_owned())
  }

  pub fn of_macro(commands: VecDeque<Command>) -> Self {
    Command::Macro(MacroCommand { commands })
  }

  pub fn of_macro_with_empty_commands() -> Self {
    Command::Macro(MacroCommand {
      commands: VecDeque::new(),
    })
  }

  pub fn as_macro(&self) -> Option<&MacroCommand> {
    match self {
      Command::Macro(m) => Some(m),
      _ => None,
    }
  }

  pub fn as_macro_mut(&mut self) -> Option<&mut MacroCommand> {
    match self {
      Command::Macro(m) => Some(m),
      _ => None,
    }
  }

  pub fn execute(&self) {
    match self {
      Command::Echo(s) => println!("{}", s),
      Command::Double(s) => println!("{}{}", s, s),
      Command::Macro(MacroCommand { commands }) => {
        for cmd in commands {
          cmd.execute();
        }
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fmt::Formatter;

  #[test]
  fn test() {
    let mut mc = Command::of_macro_with_empty_commands();
    mc.as_macro_mut().unwrap().append(Command::of_echo("Hello"));
    mc.execute();
  }
}
