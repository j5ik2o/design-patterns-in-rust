use std::fmt::{Display, Formatter};

pub trait Context {
  fn change_state(&mut self, state: State);
  fn call_security_center(&self, msg: &str);
  fn record_log(&self, msg: &str);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
  Day,
  Night,
}

impl Display for State {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      State::Night => write!(f, "[夜間]"),
      State::Day => write!(f, "[昼間]"),
    }
  }
}

impl State {
  pub fn do_clock(self, context: &mut impl Context, hour: u32) -> State {
    let new_state = match (self, (9..17).contains(&hour)) {
      (State::Night, true) | (State::Day, false) => self.opposite(),
      _ => self,
    };
    if new_state != self {
      context.change_state(new_state);
    }
    context.record_log(&format!("時刻は{}時になりました。", hour));
    context.record_log(&format!("現在の状態は{}です。", new_state));
    new_state
  }

  pub fn do_use(self, context: &impl Context) {
    match self {
      State::Night => context.call_security_center("非常：夜間の金庫使用！"),
      State::Day => context.record_log("金庫使用(昼間)"),
    }
  }

  pub fn do_alarm(self, context: &impl Context) {
    let msg = match self {
      State::Night => "非常ベル(夜間)",
      State::Day => "非常ベル(昼間)",
    };
    context.call_security_center(msg);
  }

  pub fn do_phone(self, context: &impl Context) {
    let msg = match self {
      State::Night => "夜間の通話録音",
      State::Day => "通常の通話(昼間)",
    };
    context.record_log(msg);
  }

  fn opposite(self) -> Self {
    match self {
      State::Day => State::Night,
      State::Night => State::Day,
    }
  }
}

struct StateContext {
  state: State,
}

impl StateContext {
  fn new(state: State) -> Self {
    Self { state }
  }

  fn run(&mut self) {
    for hour in 0..=24 {
      self.state = self.state.do_clock(self, hour);
      match hour % 3 {
        0 => self.state.do_use(self),
        1 => self.state.do_alarm(self),
        2 => self.state.do_phone(self),
        _ => unreachable!(),
      }
    }
  }
}

impl Context for StateContext {
  fn change_state(&mut self, state: State) {
    self.state = state;
  }

  fn call_security_center(&self, msg: &str) {
    println!("{}:{}", self.state, msg);
  }

  fn record_log(&self, msg: &str) {
    println!("{}:{}", self.state, msg);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let mut context = StateContext::new(State::Day);
    context.run();
  }
}
