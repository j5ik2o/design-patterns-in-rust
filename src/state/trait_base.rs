use std::fmt::{Display, Formatter};

pub trait Context {
  fn set_clock(&mut self, hour: u32);
  fn change_state(&mut self, state: &'static dyn State);
  fn call_security_center(&self, msg: &str);
  fn record_log(&self, msg: &str);
}

pub trait State: Display {
  fn do_clock(&self, context: &mut dyn Context, hour: u32);
  fn do_use(&self, context: &dyn Context);
  fn do_alarm(&self, context: &dyn Context);
  fn do_phone(&self, context: &dyn Context);
}

struct Day;

impl Display for Day {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[昼間]")
  }
}

impl State for Day {
  fn do_clock(&self, context: &mut dyn Context, hour: u32) {
    if !(9..17).contains(&hour) {
      context.change_state(&NIGHT);
    }
  }

  fn do_use(&self, context: &dyn Context) {
    context.record_log("金庫使用(昼間)");
  }

  fn do_alarm(&self, context: &dyn Context) {
    context.call_security_center("非常ベル(昼間)");
  }

  fn do_phone(&self, context: &dyn Context) {
    context.record_log("通常の通話(昼間)");
  }
}

struct Night;

impl Display for Night {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[夜間]")
  }
}

impl State for Night {
  fn do_clock(&self, context: &mut dyn Context, hour: u32) {
    if (9..17).contains(&hour) {
      context.change_state(&DAY);
    }
  }

  fn do_use(&self, context: &dyn Context) {
    context.call_security_center("非常：夜間の金庫使用！");
  }

  fn do_alarm(&self, context: &dyn Context) {
    context.call_security_center("非常ベル(夜間)");
  }

  fn do_phone(&self, context: &dyn Context) {
    context.record_log("夜間の通話録音");
  }
}

static DAY: Day = Day;
static NIGHT: Night = Night;

struct StateContext {
  state: &'static dyn State,
}

impl StateContext {
  fn new(state: &'static dyn State) -> Self {
    Self { state }
  }

  fn run(&mut self) {
    for hour in 0..=24 {
      self.set_clock(hour);
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
  fn set_clock(&mut self, hour: u32) {
    self.state.do_clock(self, hour);
  }

  fn change_state(&mut self, state: &'static dyn State) {
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
    let mut context = StateContext::new(&DAY);
    context.run();
  }
}