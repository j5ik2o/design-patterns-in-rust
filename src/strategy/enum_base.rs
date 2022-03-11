use crate::strategy::Hand;
use rand::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Strategy {
  Winning {
    rng: ThreadRng,
    won: bool,
    prev_hand: Option<Hand>,
  },
  Probe {
    rng: ThreadRng,
    prev_hand_value: u32,
    current_hand_value: u32,
    history: [[u32; 3]; 3],
  },
}

impl Strategy {
  pub fn of_winning() -> Self {
    let rng: ThreadRng = rand::thread_rng();
    Strategy::Winning {
      rng,
      won: false,
      prev_hand: None,
    }
  }

  pub fn of_probe() -> Self {
    let rng: ThreadRng = rand::thread_rng();
    Strategy::Probe {
      rng,
      prev_hand_value: 0,
      current_hand_value: 0,
      history: [[1; 3]; 3],
    }
  }

  fn get_sum(history: &[[u32; 3]; 3], hand_value: u32) -> u32 {
    let mut result = 0;
    for i in 0..2 {
      result += history[hand_value as usize][i as usize]
    }
    result
  }

  pub fn next_hand(&mut self) -> Option<Hand> {
    match self {
      Strategy::Winning {
        rng, won, prev_hand, ..
      } => {
        if !*won {
          *prev_hand = Some(Hand::get_hand(rng.gen_range(0, 2)))
        }
        prev_hand.clone()
      }
      Strategy::Probe {
        rng,
        prev_hand_value,
        current_hand_value,
        history,
        ..
      } => {
        let bet = rng.gen_range(0, Self::get_sum(history, *current_hand_value));
        let hand_value = if bet < history[*current_hand_value as usize][0] {
          0
        } else if bet < history[*current_hand_value as usize][0] + history[*current_hand_value as usize][1] {
          1
        } else {
          2
        };
        *prev_hand_value = *current_hand_value;
        *current_hand_value = hand_value;
        Some(Hand::get_hand(hand_value))
      }
    }
  }

  pub fn study(&mut self, win: bool) {
    match self {
      Strategy::Winning { won, .. } => *won = win,
      Strategy::Probe {
        prev_hand_value,
        current_hand_value,
        history,
        ..
      } => {
        if win {
          history[*prev_hand_value as usize][*current_hand_value as usize] += 1;
        } else {
          history[*prev_hand_value as usize][((*current_hand_value + 1) % 3) as usize] += 1;
          history[*prev_hand_value as usize][((*current_hand_value + 2) % 3) as usize] += 1;
        }
      }
    }
  }
}

#[derive(Debug)]
pub struct Player {
  name: String,
  strategy: Strategy,
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
  pub fn new(name: &str, strategy: Strategy) -> Self {
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
    let winning_strategy = Strategy::of_winning();
    let probe_strategy = Strategy::of_probe();

    let mut player1 = Player::new("Taro", winning_strategy);
    let mut player2 = Player::new("Hana", probe_strategy);

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
