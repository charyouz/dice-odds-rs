#![allow(dead_code)]

use crate::dice::{FullRoll, Roll, Die};
use core::result::Result;
use crate::die_errors::CalcError;

pub(crate) fn calculate_odds(roll: &FullRoll) {
    let _total_dice = roll.total_dice;
    for _roll_ in &roll.rolls {
        todo!();
    };
}


/// Calculate the odd of one die being the required number.
pub(crate) fn calculate_die_odd(die: &Die) -> Result<f64, CalcError> {
    let odds: f64;
    let above: f64;
    if die.req_value > usize::from(die.size) && die.above_below == "+" {
        return Err(CalcError::InvalidMaxValue)
    }

    else if die.req_value < 1 && die.above_below == "-" {
        return Err(CalcError::InvalidMinValue);
    }

    let size = usize::from(die.size);
    if die.above_below == "+" {
        above = (size - die.req_value + 1) as f64;
    }
    else if die.above_below == "-"{
         above = die.req_value as f64;
    }
    else {
        above = 0.0;
    }
    odds = above / size as f64;

    Ok(odds)
}


/// Calculate the odds for a roll where all the dice have the same requirements.
pub(crate) fn calculate_roll_odds(roll: &Roll) -> f64 {
    let dice_amount = roll.amount;
    let die_odd = calculate_die_odd(&roll.dice).unwrap();
    let odds = die_odd.powi(dice_amount.get().into());
    odds
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::dice::{DiceSize, DieBuilder, RollBuilder};
    use std::num::NonZeroU8;

    #[test]
    fn test_calculate_die_odd() {
        // Check that simple calculations work properly
        let test_die = Die {
            size: DiceSize::D6,
            req_value: 4,
            above_below: "+".to_string(),
        };
        assert_eq!(calculate_die_odd(&test_die).unwrap(), 0.5);
        let test_die2 = Die {
            size: DiceSize::D6,
            req_value: 3,
            above_below: "-".to_string(),
        };
        assert_eq!(calculate_die_odd(&test_die2).unwrap(), 0.5);

        // Check that Errors work properly
        let faulty_die = Die {
            size: DiceSize::D6,
            req_value: 7,
            above_below: "+".to_string(),
        };
        assert_eq!(calculate_die_odd(&faulty_die), Err(CalcError::InvalidMaxValue));
        let faulty_die2 = Die {
            size: DiceSize::D6,
            req_value: 0,
            above_below: "-".to_string(),
        };
        assert_eq!(calculate_die_odd(&faulty_die2), Err(CalcError::InvalidMinValue));
    }

    #[test]
    fn test_calculate_roll_odds() {
        let test_die = DieBuilder::default().size(DiceSize::D6).req_value(4).above_below("+".to_string()).build().unwrap();
        let test_roll = RollBuilder::default().dice(test_die).amount(NonZeroU8::new(2).unwrap()).build().unwrap();
        assert_eq!(calculate_roll_odds(&test_roll), 0.25);
    }

}
