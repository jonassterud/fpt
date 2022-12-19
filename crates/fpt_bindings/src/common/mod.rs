use anyhow::{anyhow, Result};

/// Get value from `serde_json::Value` as a `&serde_json::Value`.
pub fn gv<'a>(key: &str, val: &'a serde_json::Value) -> Result<&'a serde_json::Value> {
    val.get(key)
        .ok_or_else(|| anyhow!("value is missing {key}"))
}

/// Get value from `serde_json::Value` as a `&str`.
pub fn gv_str<'a>(key: &str, val: &'a serde_json::Value) -> Result<&'a str> {
    gv(key, val)?
        .as_str()
        .ok_or_else(|| anyhow!("value is not a str"))
}

/// Get value from `serde_json::Value` as a `&Vec<serde_json::Value>`.
pub fn gv_vec<'a>(key: &str, val: &'a serde_json::Value) -> Result<&'a Vec<serde_json::Value>> {
    gv(key, val)?
        .as_array()
        .ok_or_else(|| anyhow!("value is not an array"))
}

/// Get value from `serde_json::Value` as a `&f64`.
pub fn gv_f64(key: &str, val: &serde_json::Value) -> Result<f64> {
    gv(key, val)?
        .as_f64()
        .ok_or_else(|| anyhow!("value is not a f64"))
}
