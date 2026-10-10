//! 설정 페이지가 localStorage에 저장한 핸들링 값을 엔진 설정으로 바꾼다.

use tetrust_engine::{Config, Millis};

use crate::lib::constants::setting::INFINITY;

/// 저장된 값이 없거나 읽을 수 없으면 엔진 기본값을 쓴다.
pub fn load_config() -> Config {
    let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten());
    let get = |key: &str| {
        storage
            .as_ref()
            .and_then(|s| s.get_item(key).ok().flatten())
    };

    config_from(get("das"), get("arr"), get("sdf"))
}

/// - DAS, ARR: 밀리초.
/// - SDF(Soft Drop Factor): 소프트 드롭이 중력보다 몇 배 빠른지. `∞`면 바닥까지 즉시.
pub fn config_from(das: Option<String>, arr: Option<String>, sdf: Option<String>) -> Config {
    let mut config = Config::default();
    let millis = |value: Option<String>| value.and_then(|v| v.trim().parse::<Millis>().ok());

    if let Some(das) = millis(das) {
        config.das = das;
    }
    if let Some(arr) = millis(arr) {
        config.arr = arr;
    }
    if let Some(sdf) = sdf {
        let sdf = sdf.trim();
        if sdf.starts_with(INFINITY) {
            config.soft_drop = 0;
        } else if let Some(soft_drop) = sdf
            .parse::<Millis>()
            .ok()
            .and_then(|factor| config.gravity.checked_div(factor))
        {
            config.soft_drop = soft_drop.max(1);
        }
    }

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> Option<String> {
        Some(v.to_string())
    }

    #[test]
    fn missing_values_keep_engine_defaults() {
        assert_eq!(config_from(None, None, None), Config::default());
        assert_eq!(config_from(s("abc"), s(""), s("0")), Config::default());
    }

    #[test]
    fn reads_handling_values() {
        let config = config_from(s("300"), s("0"), s("5"));
        assert_eq!(config.das, 300);
        assert_eq!(config.arr, 0);
        assert_eq!(config.soft_drop, Config::default().gravity / 5);
    }

    #[test]
    fn infinite_sdf_drops_instantly() {
        assert_eq!(config_from(None, None, s("∞")).soft_drop, 0);
    }
}
