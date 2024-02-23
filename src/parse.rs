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

#[derive(Debug, PartialEq)]
pub(crate) struct Die {
    pub size: DiceSize,
    pub req_value: u8,
    pub above_below: String,
}


#[derive(Debug, PartialEq)]
pub(crate) struct Roll {
    pub dice: Die,
    pub amount: NonZeroU8,
}

pub(crate) fn parse_dice_str(dice_str: &str) -> Result<Roll, ParseError> {
    let dice_regex = Regex::new(r"^([1-9]\d*)?d?(\d+)(\+?\-?)$").unwrap();
    let caps = dice_regex.captures(dice_str).ok_or(ParseError::UnableToParse)?;
    let dice_amount = caps.get(1)
        .ok_or(ParseError::InvalidDicenumber)?
        .as_str().parse::<NonZeroU8>()
        .map_err(|_| {ParseError::InvalidDicenumber})?;
    let dice_req = caps.get(2)
        .ok_or(ParseError::InvalidDiceSize)?
        .as_str()
        .parse::<u8>();
    let dice_min_max = caps.get(3)
        .ok_or(ParseError::UnableToParse)?
        .as_str()
        .parse::<String>();

    Ok(Roll {
        amount: dice_amount,
        dice: Die {
            size: DiceSize::from_str("6").unwrap(),
            req_value: dice_req.unwrap(),
            above_below: dice_min_max.unwrap(),
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
        let foo2 = parse_dice_str("3d5+").unwrap();
        assert_eq!(foo, foo2);
    }
}
