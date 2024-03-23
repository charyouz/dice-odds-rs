#![allow(dead_code)]

use crate::parse::Roll;
use core::result::Result;
use crate::die_errors::CalcError;

pub(crate) fn calculate_expected_amount(roll: &Roll) -> Result<f64, CalcError> {
    let odds: f64;
    let above: f64;
    if roll.dice.req_value > usize::from(roll.dice.size) && roll.dice.above_below == "+" {
        return Err(CalcError::InvalidMaxValue);
    }
    else if roll.dice.req_value < 1 && roll.dice.above_below == "-" {
        return Err(CalcError::InvalidMinValue);
    }

    if roll.dice.above_below == "+" {
        above = (usize::from(roll.dice.size) - roll.dice.req_value + 1) as f64;
    }
    else if roll.dice.above_below == "-" {
        above = roll.dice.req_value as f64;
    }
    else {
        above = 0.0;
    }
    odds = f64::from(u8::from(roll.amount)) * above  / usize::from(roll.dice.size) as f64;

    Ok(odds)
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::parse::{DiceSize, Die};
    use std::num::NonZeroU8;

    #[test]
    fn test_calculate_expected_value() {
        //Check that simple calculation works
        let test_die = Die {
            size: DiceSize::D6,
            req_value: 4,
            above_below: "+".to_string(),
        };
        let mut test_roll = Roll {
            dice: test_die,
            amount: NonZeroU8::new(4).unwrap(),
            extra_info: "".to_string(),
        };
        //Test with 4 dice, 4 or more (1/2)
        assert_eq!(calculate_expected_amount(&test_roll).unwrap(), 2.0);

        //Test with 3 dice, 4 or less (2/3)
        test_roll.dice.above_below = "-".to_string();
        test_roll.amount = NonZeroU8::new(3).unwrap();
        assert_eq!(calculate_expected_amount(&test_roll).unwrap(), 2.0);

        //Test with 4 dice, 5 or less (5/6)
        test_roll.dice.req_value = 5;
        assert_eq!(calculate_expected_amount(&test_roll).unwrap(), 2.5);
    }

}
