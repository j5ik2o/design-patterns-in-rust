use std::fmt::{Display, Formatter};

use crate::strategy::Hand;
use rand::prelude::*;
use rand::Rng;

pub trait Strategy {
  fn next_hand(&mut self) -> Option<Hand>;
  fn study(&mut self, win: bool);
}

#[derive(Clone, Debug)]
pub struct WinningStrategy {
  rng: ThreadRng,
  won: bool,
  prev_hand: Option<Hand>,
}

impl Strategy for WinningStrategy {
  fn next_hand(&mut self) -> Option<Hand> {
    if !self.won {
      self.prev_hand = Some(Hand::get_hand(self.rng.gen_range(0..=2)))
    }
    self.prev_hand.clone()
  }

  fn study(&mut self, win: bool) {
    self.won = win;
  }
}

impl WinningStrategy {
  pub fn new() -> Self {
    let rng: ThreadRng = rand::thread_rng();
    Self {
      rng,
      won: false,
      prev_hand: None,
    }
  }
}

#[derive(Clone, Debug)]
pub struct ProbeStrategy {
  rng: ThreadRng,
  prev_hand_value: u32,
  current_hand_value: u32,
  history: [[u32; 3]; 3],
}

impl Strategy for ProbeStrategy {
  fn next_hand(&mut self) -> Option<Hand> {
    let bet = self.rng.gen_range(0..=self.get_sum(self.current_hand_value));
    let hand_value = if bet < self.history[self.current_hand_value as usize][0] {
      0
    } else if bet
      < self.history[self.current_hand_value as usize][0] + self.history[self.current_hand_value as usize][1]
    {
      1
    } else {
      2
    };
    self.prev_hand_value = self.current_hand_value;
    self.current_hand_value = hand_value;
    Some(Hand::get_hand(hand_value))
  }

  fn study(&mut self, win: bool) {
    if win {
      self.history[self.prev_hand_value as usize][self.current_hand_value as usize] += 1;
    } else {
      self.history[self.prev_hand_value as usize][((self.current_hand_value + 1) % 3) as usize] += 1;
      self.history[self.prev_hand_value as usize][((self.current_hand_value + 2) % 3) as usize] += 1;
    }
  }
}

impl ProbeStrategy {
  fn get_sum(&self, hand_value: u32) -> u32 {
    let mut result = 0;
    for i in 0..2 {
      result += self.history[hand_value as usize][i as usize]
    }
    result
  }

  pub fn new() -> Self {
    let rng: ThreadRng = rand::thread_rng();
    Self {
      rng,
      prev_hand_value: 0,
      current_hand_value: 0,
      history: [[1; 3]; 3],
    }
  }
}

pub struct Player {
  name: String,
  // ストラテジがPlayのみが所有するならBox、Play以外でも共有するならRc/Arcを使う
  strategy: Box<dyn Strategy>,
  win_count: u32,
  lose_count: u32,
  game_count: u32,
}

impl Display for Player {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let s = format!(
      "[{}:, {} games, {} win, {} lose]",
      self.name, self.game_count, self.win_count, self.lose_count
    );
    write!(f, "{}", s)
  }
}

impl Player {
  pub fn new(name: &str, strategy: Box<dyn Strategy>) -> Self {
    Self {
      name: name.to_owned(),
      strategy,
      win_count: 0,
      lose_count: 0,
      game_count: 0,
    }
  }

  pub fn next_hand(&mut self) -> Option<Hand> {
    self.strategy.next_hand()
  }

  pub fn win(&mut self) {
    self.strategy.study(true);
    self.win_count += 1;
    self.game_count += 1;
  }

  pub fn lose(&mut self) {
    self.strategy.study(false);
    self.lose_count += 1;
    self.game_count += 1;
  }

  pub fn even(&mut self) {
    self.game_count += 1;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test() {
    let winning_strategy = WinningStrategy::new();
    let probe_strategy = ProbeStrategy::new();

    let mut player1 = Player::new("Taro", Box::new(winning_strategy));
    let mut player2 = Player::new("Hana", Box::new(probe_strategy));

    for _ in 0..10000 {
      let next_hand1 = player1.next_hand().unwrap();
      let next_hand2 = player2.next_hand().unwrap();
      if next_hand1.is_stronger_than(next_hand2.clone()) {
        println!("Winner:{}", player1);
        player1.win();
        player2.lose();
      } else if next_hand2.is_stronger_than(next_hand1) {
        println!("Winner:{}", player2);
        player1.lose();
        player2.win();
      } else {
        player1.even();
        player2.even();
      }
    }

    println!("Total result:");
    println!("{}", player1);
    println!("{}", player2);
  }
}
