#[allow(dead_code)]

#[derive(Debug, PartialEq)]
pub(crate) enum CalcError {
    InvalidMinValue,
    InvalidMaxValue,
    InvalidDieSize,
}
