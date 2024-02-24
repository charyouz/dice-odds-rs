use crate::parse::{FullRoll, Die};
use core::result::Result;

pub(crate) fn calculate_odds(roll: &FullRoll) {
    let total_dice = roll.total_dice;
    for roll_ in &roll.rolls {
        todo!();
    };

}

pub(crate) enum CalcError {
    InvalidMinValue,
    InvalidMaxValue,
}

fn calculate_die_odd(die: &Die) -> Result<(), CalcError> {
    if die.req_value > usize::from(die.size) && die.above_below == "+" {
        return Err(CalcError::InvalidMaxValue)
    }

    else if die.req_value < usize::from(die.size) && die.above_below == "-" {
        return Err(CalcError::InvalidMinValue);
    }
    Ok(())
}
