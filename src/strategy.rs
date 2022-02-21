use rand::prelude::ThreadRng;
use rand::Rng;
use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum Hand {
    GUU,
    CHO,
    PAA,
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Hand {
    pub fn name(&self) -> &str {
        match self {
            Hand::GUU => "グー",
            Hand::CHO => "チョキ",
            Hand::PAA => "パー",
        }
    }
    pub fn get_hand(value: u32) -> Self {
        match value {
            0 => Hand::GUU,
            1 => Hand::CHO,
            2 => Hand::PAA,
            _ => panic!("not found"),
        }
    }
    fn hand_value(&self) -> i32 {
        match self {
            Hand::GUU => 0,
            Hand::CHO => 1,
            Hand::PAA => 2,
        }
    }
    fn fight(&self, h: Hand) -> i32 {
        if *self == h {
            0
        } else if (self.hand_value() + 1) % 3 == h.hand_value() {
            1
        } else {
            -1
        }
    }
    pub fn is_stronger_than(&self, h: Hand) -> bool {
        self.fight(h) == 1
    }
    pub fn is_weaker_than(&self, h: Hand) -> bool {
        self.fight(h) == -1
    }
}

pub trait Strategy {
    fn next_hand(&mut self) -> Option<&Hand>;
    fn study(&mut self, win: bool);
}

pub struct Winning {
    rng: ThreadRng,
    won: bool,
    prev_hand: Option<Hand>,
}

impl Strategy for Winning {
    fn next_hand(&mut self) -> Option<&Hand> {
        if !self.won {
            self.prev_hand = Some(Hand::get_hand(self.rng.gen_range(0, 2)))
        }
        self.prev_hand.as_ref()
    }

    fn study(&mut self, win: bool) {
        self.won = win;
    }
}

impl Winning {
    pub fn new() -> Self {
        let rng: ThreadRng = rand::thread_rng();
        Self {
            rng,
            won: false,
            prev_hand: None,
        }
    }
}

pub struct Probe {
    rng: ThreadRng,
    prev_hand_value: u32,
    current_hand_value: u32,
    history: [[u32; 3]; 3],
}

impl Strategy for Probe {
    fn next_hand(&mut self) -> Option<&Hand> {
        let bet = self.rng.gen_range(0, ge)
    }

    fn study(&mut self, win: bool) {
        todo!()
    }
}

impl Probe {
    fn get_sum(&self, hand_value: u32) -> u32 {
       for
        self.history[hand_value][i]
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

    pub fn next_hand(&mut self) -> Option<&Hand> {
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
