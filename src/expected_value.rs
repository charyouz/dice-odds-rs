#![allow(dead_code)]

use crate::dice::{Roll};
use core::result::Result;
use crate::die_errors::{ParseError, CalcError};
use regex::Regex;

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

pub fn parse_extra_info(roll: &mut Roll) -> () {
    if roll.extra_info.is_empty() {
        return
    }
    let reg = Regex::new("([a-zA-Z][0-9]?)").unwrap();
    let info = roll.extra_info.clone();
    let caps = reg.captures(&info).ok_or(ParseError::UnableToParse).unwrap();
    for i in 0..caps.len() {
        if caps.get(i).is_some(){
            let input = caps.get(i).ok_or("asdf").unwrap().as_str(); // TODO
            if input.len() == 1 {
                //Only letter
                match input {
                    "R" => roll.set_reroll_suc(true),
                    "r" => roll.set_reroll_fail(true),
                    _ => println!("Urecognized command {}", input),
                }
            }
            else {
                //Letter and number(s)
                let (letter, number) = input.split_at(0);
                match letter {
                    "R" => {
                        roll.set_reroll_suc(true);
                        roll.set_reroll_face_amount(number.parse::<i32>().unwrap().try_into().unwrap());
                    },
                    "r" => {
                        roll.set_reroll_suc(true);
                        roll.set_reroll_face_amount(number.parse::<i32>().unwrap().try_into().unwrap());
                    },
                    _ => println!("Letter: {}", letter), // TODO

            }
        }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::dice::{DiceSize, DieBuilder, RollBuilder};
    use std::num::NonZeroU8;

    #[test]
    fn test_calculate_expected_value() {
        //Check that simple calculation works
        let test_die = DieBuilder::default().size(DiceSize::D6).req_value(4).build().unwrap();
        let mut test_roll = RollBuilder::default().dice(test_die).amount(NonZeroU8::new(4).unwrap()).build().unwrap();
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
