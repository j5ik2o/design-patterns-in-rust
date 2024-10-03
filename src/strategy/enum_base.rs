use rand::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub enum Strategy {
  Winning {
    rng: ThreadRng,
    won: bool,
    prev_hand: Hand,
  },
  Probe {
    rng: ThreadRng,
    prev_hand_value: u8,
    current_hand_value: u8,
    history: [[u32; 3]; 3],
  },
}

impl Strategy {
  pub fn of_winning() -> Self {
    Strategy::Winning {
      rng: rand::thread_rng(),
      won: false,
      prev_hand: Hand::Rock,
    }
  }

  pub fn of_probe() -> Self {
    Strategy::Probe {
      rng: rand::thread_rng(),
      prev_hand_value: 0,
      current_hand_value: 0,
      history: [[1; 3]; 3],
    }
  }

  fn get_sum(history: &[[u32; 3]; 3], hand_value: u8) -> u32 {
    history[hand_value as usize].iter().sum()
  }

  pub fn next_hand(&mut self) -> Hand {
    match self {
      Strategy::Winning { rng, won, prev_hand } => {
        if !*won {
          *prev_hand = Hand::get_hand(rng.gen_range(0..=2));
        }
        *prev_hand
      }
      Strategy::Probe {
        rng,
        prev_hand_value,
        current_hand_value,
        history,
      } => {
        let bet = rng.gen_range(0..=Self::get_sum(history, *current_hand_value));
        let hand_value = if bet < history[*current_hand_value as usize][0] {
          0
        } else if bet < history[*current_hand_value as usize][0] + history[*current_hand_value as usize][1] {
          1
        } else {
          2
        };
        *prev_hand_value = *current_hand_value;
        *current_hand_value = hand_value;
        Hand::get_hand(hand_value)
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
    write!(
      f,
      "[{}: {} games, {} win, {} lose]",
      self.name, self.game_count, self.win_count, self.lose_count
    )
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
    let mut player1 = Player::new("Taro", Strategy::of_winning());
    let mut player2 = Player::new("Hana", Strategy::of_probe());

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
