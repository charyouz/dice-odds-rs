#[allow(dead_code)]

#[derive(Debug, PartialEq)]
pub(crate) enum CalcError {
    InvalidMinValue,
    InvalidMaxValue,
    InvalidDieSize,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParseError {
    InvalidDicenumber,
    InvalidDiceSize,
    UnableToParse,
}

#[derive(Debug, PartialEq)]
pub(crate) enum DieError {
    InvalidSetting,
}
