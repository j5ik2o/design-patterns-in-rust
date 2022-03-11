use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub trait Context {
  fn set_clock(&mut self, hour: u32);
  fn change_state(&mut self, state: Rc<dyn State<Self>>);
  fn call_security_center(&self, msg: &str);
  fn record_log(&self, msg: &str);
}

pub trait State<C>: Display {
  fn do_clock(&self, context: &mut C, hour: u32);
  fn do_use(&self, context: &C);
  fn do_alarm(&self, context: &C);
  fn do_phone(&self, context: &C);
}

#[derive(Debug)]
pub struct Day;

impl Display for Day {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[昼間]")
  }
}

impl<C: Context> State<C> for Day {
  fn do_clock(&self, context: &mut C, hour: u32) {
    if hour < 9 || 17 <= hour {
      context.change_state(Rc::new(Night));
    }
  }

  fn do_use(&self, context: &C) {
    context.record_log("金庫使用(昼間)");
  }

  fn do_alarm(&self, context: &C) {
    context.call_security_center("非常ベル(昼間)");
  }

  fn do_phone(&self, context: &C) {
    context.record_log("通常の通話(昼間)");
  }
}

pub struct Night;

impl Display for Night {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[夜間]")
  }
}

impl<C: Context> State<C> for Night {
  fn do_clock(&self, context: &mut C, hour: u32) {
    if 9 <= hour && hour < 17 {
      context.change_state(Rc::new(Day));
    }
  }

  fn do_use(&self, context: &C) {
    context.call_security_center("非常：夜間の金庫使用！");
  }

  fn do_alarm(&self, context: &C) {
    context.call_security_center("非常ベル(夜間)");
  }

  fn do_phone(&self, context: &C) {
    context.record_log("夜間の通話録音");
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[derive(Clone)]
  struct StateContext {
    state: Rc<dyn State<StateContext>>,
  }

  impl StateContext {
    fn new(state: Rc<dyn State<StateContext>>) -> Self {
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

    fn change_state(&mut self, state: Rc<dyn State<StateContext>>) {
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
    let state = Rc::new(Day);
    let mut context = StateContext::new(state);
    context.run();
  }
}
