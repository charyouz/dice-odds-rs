use std::num::NonZeroU8
use std::str::FromStr

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

pub(crate) struct Die {
    pub size: DiceSize,
    pub req_value: u8,
    pub above_below: String,
}


pub(crate) struct Roll {
    pub dice: Vec<Die>,
}

pub(crate) fn parse_dice_str(dice_str: &str) -> Result<Die, ParseError> {
    let dice_regex = Regex::new(r"^([1-9]\d*)?d?(\d+)(\+?\-?)$").unwrap();
    let caps = dice_regex.captures(dice_str).ok_or(ParseError::UnableToParse)?;
    let dice_num = caps.get(1)
        .ok_or(ParseError::InvalidDicenumber)?
        .as_str().parse::<NonZeroU8>()
        .map_err(|_| {ParseError::InvalidDicenumber})?;
    let dice_size = caps.get(2)
        .ok_or(ParseError::InvalidDiceSize)?
        .as_str()
        .parse::<DiceSize>()?;
    let dice_min_max = caps.get(3)
        .ok_or(ParseError::UnableToParse)?
        .as_str()
        .parse<String>()?;

    Ok(Die {
        size: dice_num,
        req_value: dice_size,
        above_below: dice_min_max,
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dice_str() {
        todo!()
    }
}
