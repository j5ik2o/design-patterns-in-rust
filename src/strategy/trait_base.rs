use rand::prelude::*;
use rand::Rng;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub enum Hand {
  Rock,
  Paper,
  Scissors,
}

impl Hand {
  fn get_hand(value: u8) -> Self {
    match value {
      0 => Hand::Rock,
      1 => Hand::Paper,
      _ => Hand::Scissors,
    }
  }

  fn is_stronger_than(&self, other: Hand) -> bool {
    matches!(
      (self, other),
      (Hand::Rock, Hand::Scissors) | (Hand::Scissors, Hand::Paper) | (Hand::Paper, Hand::Rock)
    )
  }
}

pub trait Strategy {
  fn next_hand(&mut self) -> Hand;
  fn study(&mut self, win: bool);
}

#[derive(Clone, Debug)]
pub struct WinningStrategy {
  rng: ThreadRng,
  won: bool,
  prev_hand: Hand,
}

impl Strategy for WinningStrategy {
  fn next_hand(&mut self) -> Hand {
    if !self.won {
      self.prev_hand = Hand::get_hand(self.rng.gen_range(0..=2))
    }
    self.prev_hand
  }

  fn study(&mut self, win: bool) {
    self.won = win;
  }
}

impl WinningStrategy {
  pub fn new() -> Self {
    Self {
      rng: rand::thread_rng(),
      won: false,
      prev_hand: Hand::Rock,
    }
  }
}

#[derive(Clone, Debug)]
pub struct ProbeStrategy {
  rng: ThreadRng,
  prev_hand_value: u8,
  current_hand_value: u8,
  history: [[u32; 3]; 3],
}

impl Strategy for ProbeStrategy {
  fn next_hand(&mut self) -> Hand {
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
    Hand::get_hand(hand_value)
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
  fn get_sum(&self, hand_value: u8) -> u32 {
    self.history[hand_value as usize].iter().sum()
  }

  pub fn new() -> Self {
    Self {
      rng: rand::thread_rng(),
      prev_hand_value: 0,
      current_hand_value: 0,
      history: [[1; 3]; 3],
    }
  }
}

pub struct Player {
  name: String,
  strategy: Box<dyn Strategy>,
  win_count: u32,
  lose_count: u32,
  game_count: u32,
}

impl Display for Player {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}: {} games, {} win, {} lose]",
      self.name, self.game_count, self.win_count, self.lose_count
    )
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

  pub fn next_hand(&mut self) -> Hand {
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
    let mut player1 = Player::new("Taro", Box::new(WinningStrategy::new()));
    let mut player2 = Player::new("Hana", Box::new(ProbeStrategy::new()));

    for _ in 0..10000 {
      let next_hand1 = player1.next_hand();
      let next_hand2 = player2.next_hand();
      if next_hand1.is_stronger_than(next_hand2) {
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
