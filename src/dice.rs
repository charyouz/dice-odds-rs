#![allow(dead_code)]
use std::str::FromStr;
use derive_builder::Builder;
use crate::die_errors::{ParseError, DieError};
use core::num::NonZeroU8;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum DiceSize {
    D3,
    D6,
}

impl FromStr for DiceSize {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "3" => Ok(DiceSize::D3),
            "6" => Ok(DiceSize::D6),
            _ => Err(ParseError::InvalidDiceSize)
            }
        }
}

/// Different die sizes
impl From<DiceSize> for usize {
    fn from(d: DiceSize) -> Self {
        match d {
            DiceSize::D3 => 3,
            DiceSize::D6 => 6,
        }
    }
}

/// A single die, with its' size (amount of faces), requirement value, and if the result shuld be above or below the value.
#[derive(Debug, PartialEq, Clone, Builder)]
pub(crate) struct Die {
    pub size: DiceSize,
    #[builder(default="1")]
    pub req_value: usize,
    #[builder(default="\"+\".to_string()")]
    pub above_below: String,
}


/// A roll of dice, where the dice in it should be the same, e.g. 3 dice that need to be 4 or more.
#[derive(Debug, PartialEq, Builder)]
pub(crate) struct Roll {
    pub dice: Die, //Type of die
    #[builder(default="NonZeroU8::new(1).unwrap()")]
    pub amount: NonZeroU8, //How many dice
    #[builder(default="\"\".to_string()")]
    pub extra_info: String, //Extra info, to check later
    #[builder(default="0")]
    pub re_rolls: usize, //How many faces to re-roll (e.g. only roll 1's = 1, re-roll only 1's and 2's =2, re-roll 6's = 1)
    #[builder(default="false")]
    pub re_roll_suc: bool, // Re-roll successes?
    #[builder(default="false")]
    pub re_roll_fail: bool, // Re-roll failures?
}

impl Roll {
    pub fn set_reroll_suc(&mut self, value: bool) {
        self.re_roll_suc = value;
    }
    pub fn set_reroll_fail(&mut self, value: bool) {
        self.re_roll_fail = value;
    }
    pub fn set_reroll_face_amount(&mut self, value: usize) {
        self.re_rolls = value;
    }
    pub fn check_reroll_ok(self) -> Result<(), DieError> {
        if self.re_roll_fail && self.re_roll_suc {
            return Err(DieError::InvalidSetting)
        }
        Ok(())
    }
}

/// All of the dice in the roll
#[derive(Debug, PartialEq)]
pub(crate) struct FullRoll {
    pub rolls: Vec<Roll>,
    pub total_dice: NonZeroU8,
}
