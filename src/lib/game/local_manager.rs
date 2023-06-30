use crate::lib::{
    constants::setting::INFINITY,
    js_bind::localstorage::{get_local_value, set_local_value},
};

use super::sdf::sdf_value::SdfValue;

pub fn get_sdf_or_set_default() -> SdfValue {
    if let None = get_local_value("sdf") {
        set_local_value("sdf", 5.to_string());
    }

    get_local_value("sdf")
        .map(|v| {
            if v == INFINITY.to_string() {
                Some(SdfValue::Infinity)
            } else {
                Some(SdfValue::Number(v.parse::<u32>().ok()?))
            }
        })
        .flatten()
        .unwrap_or(SdfValue::Number(5))
}

pub fn set_sdf(value: SdfValue) {
    let value = match value {
        SdfValue::Infinity => INFINITY.to_string(),
        SdfValue::Number(v) => v.to_string(),
    };

    set_local_value("sdf", value);
}

pub fn get_arr_or_set_default() -> u32 {
    if let None = get_local_value("arr") {
        set_local_value("arr", 0.to_string());
    }

    get_local_value("arr")
        .map(|v| v.parse::<u32>().ok())
        .flatten()
        .unwrap_or(0)
}

pub fn set_arr(value: u32) {
    set_local_value("arr", value.to_string());
}

pub fn get_das_or_set_default() -> u32 {
    if let None = get_local_value("das") {
        set_local_value("das", 300.to_string());
    }

    get_local_value("das")
        .map(|v| v.parse::<u32>().ok())
        .flatten()
        .unwrap_or(300)
}

pub fn set_das(value: u32) {
    set_local_value("das", value.to_string());
}
