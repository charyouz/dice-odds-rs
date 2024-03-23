#![allow(dead_code)]

use std::str::FromStr;
use std::num::NonZeroU8;
use regex::Regex;
use crate::die_errors::ParseError;
use crate::dice::{DiceSize, Roll, DieBuilder, RollBuilder};

/// Parses string to dice and roll.
/// Input should be in the format <number of dice>x<dice face number>d<wanted number><+/- if the wanted number is higher or lower>
// After this, there can be anything behind an underscore, this will be tested later
pub(crate) fn parse_dice_str(dice_str: &str) -> Result<Roll, ParseError> {
    let dice_amount: NonZeroU8;
    let dice_sides: String;
    let dice_min_max: String;
    let ext_inf: String;

    let dice_regex = Regex::new(r"([1-9]\d*)?x?([1-9]\d*)?d?(\d+)(\+?\-?)_?(.*)?").unwrap();
    let caps = dice_regex.captures(dice_str).ok_or(ParseError::UnableToParse)?;
    if caps.get(1).is_none() {
    dice_amount = NonZeroU8::new(1).unwrap();
    } else {
        dice_amount = caps.get(1)
            .ok_or(ParseError::InvalidDicenumber)?
            .as_str().parse::<NonZeroU8>()
            .map_err(|_| {ParseError::InvalidDicenumber})?;
    }
    if caps.get(2).is_none() {
        dice_sides = "6".to_string(); //Defaults to D6, maybe should be handled somehow else?
    } else {
        dice_sides = caps.get(2)
            .ok_or(ParseError::InvalidDiceSize)?
            .as_str().parse::<String>()
            .map_err(|_| {ParseError::InvalidDicenumber})?;
    }
    let dice_req = caps.get(3)
        .ok_or(ParseError::InvalidDiceSize)?
        .as_str()
        .parse::<usize>();
    if caps.get(4).is_none() {
        dice_min_max = "+".to_string();
    }
    else {
        dice_min_max = caps.get(4)
            .ok_or(ParseError::UnableToParse)?
            .as_str()
            .parse::<String>().unwrap();
    }
    if caps.get(5).is_none(){
        ext_inf = "".to_string();
    }
    else {
        ext_inf = caps.get(5)
        .ok_or(ParseError::UnableToParse)?
        .as_str()
        .parse::<String>().unwrap();
    }

    //Build output struct
    let output_die = DieBuilder::default()
        .size(DiceSize::from_str(&dice_sides).unwrap())
        .req_value(dice_req.unwrap())
        .above_below(dice_min_max)
        .build().unwrap();
    let output = RollBuilder::default()
        .amount(dice_amount)
        .dice(output_die)
        .extra_info(ext_inf.clone())
        .build().unwrap();

    Ok(output)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dice_str() {
        let mut test_die = DieBuilder::default().size(DiceSize::from_str("6").unwrap()).req_value(5).above_below("+".to_string()).build().unwrap();
        let foo = RollBuilder::default().amount(NonZeroU8::new(3).unwrap()).dice(test_die.clone()).build().unwrap();
        let foo2 = parse_dice_str("3x6d5+").unwrap();
        assert_eq!(foo, foo2);
        let foo3 = parse_dice_str("3x5+").unwrap();
        assert_eq!(foo, foo3);
        test_die.req_value = 3;
        test_die.above_below = "-".to_string();
        let foo4 = RollBuilder::default().amount(NonZeroU8::new(1).unwrap()).dice(test_die.clone()).build().unwrap();
        assert_eq!(foo4, parse_dice_str("3-").unwrap());
    }
}
