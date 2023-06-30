use crate::lib::constants::setting::INFINITY;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SdfValue {
    Infinity,
    Number(u32),
}

impl ToString for SdfValue {
    fn to_string(&self) -> String {
        match self {
            SdfValue::Infinity => INFINITY.to_string(),
            SdfValue::Number(v) => v.to_string(),
        }
    }
}
