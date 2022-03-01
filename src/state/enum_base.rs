use std::fmt::{Display, Formatter};

pub trait Context {
  fn set_clock(&mut self, hour: u32);
  fn change_state(&mut self, state: State);
  fn call_security_center(&self, msg: &str);
  fn record_log(&self, msg: &str);
}

#[derive(Clone)]
pub enum State {
  Day,
  Night,
}

impl Display for State {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      State::Night => "[夜間]",
      State::Day => "[昼間]",
    };
    write!(f, "{}", s)
  }
}

impl State {
  pub fn do_clock<C: Context>(&self, context: &mut C, hour: u32) {
    match self {
      State::Night => {
        if 9 <= hour && hour < 17 {
          context.change_state(State::Day);
        }
      }
      State::Day => {
        if hour < 9 || 17 <= hour {
          context.change_state(State::Night);
        }
      }
    }
  }

  pub fn do_use<C: Context>(&self, context: &C) {
    match self {
      State::Night => context.call_security_center("非常：夜間の金庫使用！"),
      State::Day => context.record_log("金庫使用(昼間)"),
    }
  }

  pub fn do_alarm<C: Context>(&self, context: &C) {
    match self {
      State::Night => context.call_security_center("非常ベル(夜間)"),
      State::Day => context.call_security_center("非常ベル(昼間)"),
    }
  }

  pub fn do_phone<C: Context>(&self, context: &C) {
    match self {
      State::Night => context.record_log("夜間の通話録音"),
      State::Day => context.record_log("通常の通話(昼間)"),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[derive(Clone)]
  struct StateContext {
    state: State,
  }

  impl StateContext {
    fn new(state: State) -> Self {
      Self { state }
    }

    fn run(&mut self) {
      let mut i = 0;
      for hour in 0..=24 {
        self.set_clock(hour);
        match i {
          0 => {
            self.state.do_use(self);
            i = 1;
          }
          1 => {
            self.state.do_alarm(self);
            i = 2;
          }
          2 => {
            self.state.do_phone(self);
            i = 0;
          }
          _ => panic!("iae"),
        }
      }
    }
  }

  impl Context for StateContext {
    fn set_clock(&mut self, hour: u32) {
      let mut current_state = self.state.clone();
      current_state.do_clock(self, hour);
    }

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

  #[test]
  fn test() {
    let state = State::Day;
    let mut context = StateContext::new(state);
    context.run();
  }
}
