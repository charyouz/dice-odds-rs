use std::num::NonZeroU8;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub(crate) enum ParseError {
    InvalidDicenumber,
    InvalidDiceSize,
    UnableToParse,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum DiceSize {
    D6,
}

impl FromStr for DiceSize {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "6" => Ok(DiceSize::D6),
            _ => Err(ParseError::InvalidDiceSize)
            }
        }
}

impl From<DiceSize> for usize {
    fn from(d: DiceSize) -> Self {
        match d {
            DiceSize::D6 => 6,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Die {
    pub size: DiceSize,
    pub req_value: usize,
    pub above_below: String,
}


#[derive(Debug, PartialEq)]
pub(crate) struct Roll {
    pub dice: Die,
    pub amount: NonZeroU8,
}

#[derive(Debug, PartialEq)]
pub(crate) struct FullRoll {
    pub rolls: Vec<Roll>,
    pub total_dice: NonZeroU8,
}

pub(crate) fn parse_dice_str(dice_str: &str) -> Result<Roll, ParseError> {
    let dice_amount: NonZeroU8;
    let dice_sides: String;
    let dice_min_max: String;

    let dice_regex = Regex::new(r"^([1-9]\d*)?x?([1-9]\d*)?d?(\d+)(\+?\-?)$").unwrap();
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
        dice_sides = "6".to_string();
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
    } else {
        dice_min_max = caps.get(4)
            .ok_or(ParseError::UnableToParse)?
            .as_str()
            .parse::<String>().unwrap();
    }

    Ok(Roll {
        amount: dice_amount,
        dice: Die {
            size: DiceSize::from_str(&dice_sides).unwrap(),
            req_value: dice_req.unwrap(),
            above_below: dice_min_max,
    }
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dice_str() {
        let foo = Roll {
            amount: NonZeroU8::new(3).unwrap(),
            dice: Die {
                size: DiceSize::from_str("6").unwrap(),
                req_value: 5,
                above_below: "+".to_string(),
            }
        };
        let foo2 = parse_dice_str("3x6d5+").unwrap();
        assert_eq!(foo, foo2);
        let foo3 = parse_dice_str("3x5+").unwrap();
        assert_eq!(foo, foo3);
        let foo4 = Roll {
            amount: NonZeroU8::new(1).unwrap(),
            dice: Die {
                size: DiceSize::from_str("6").unwrap(),
                req_value: 3,
                above_below: "-".to_string(),
            }
        };
        assert_eq!(foo4, parse_dice_str("3-").unwrap());
    }
}
