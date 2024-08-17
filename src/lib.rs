// MIT License

// Copyright (c) 2024 podzito

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::anyhow;
use anyhow::Result;
use serde_json::Map;
use serde_json::Value;

pub mod model;
pub mod util;

fn get_obj<'a>(obj: &'a Value, key: &str) -> Result<&'a Map<String, Value>> {
    let value = obj
        .get(key)
        .ok_or(anyhow!(format!("Missing key: {}", key)))?;
    value
        .as_object()
        .ok_or(anyhow!(format!("Expected object: {}", key)))
}

fn get_str(obj: &Value, key: &str) -> Result<String> {
    Ok(obj
        .get(key)
        .ok_or(anyhow!(format!("Missing key: {}", key)))?
        .as_str()
        .ok_or(anyhow!(format!("Expected string: {}", key)))?
        .to_string())
}

fn get_optional_str(obj: &Map<String, Value>, key: &str) -> Result<Option<String>> {
    Ok(obj
        .get(key)
        .map(|v| {
            v.as_str()
                .ok_or(anyhow!(format!("Expected string: {}", key)))
                .map(|v| v.to_string())
                .ok()
        })
        .unwrap_or(None))
}

fn get_i64(obj: &Map<String, Value>, key: &str) -> Result<i64> {
    Ok(obj
        .get(key)
        .ok_or(anyhow!(format!("Missing key: {}", key)))?
        .as_i64()
        .ok_or(anyhow!(format!("Expected i64: {}", key)))?)
}

fn get_optional_i64(obj: &Map<String, Value>, key: &str) -> Result<Option<i64>> {
    Ok(obj
        .get(key)
        .map(|v| {
            v.as_i64()
                .ok_or(anyhow!(format!("Expected i64: {}", key)))
                .ok()
        })
        .unwrap_or(None))
}

fn get_bool(obj: &Map<String, Value>, key: &str) -> Result<bool> {
    Ok(obj
        .get(key)
        .ok_or(anyhow!(format!("Missing key: {}", key)))?
        .as_bool()
        .ok_or(anyhow!(format!("Expected bool: {}", key)))?)
}

fn get_optional_bool(obj: &Map<String, Value>, key: &str) -> Result<Option<bool>> {
    Ok(obj
        .get(key)
        .map(|v| {
            v.as_bool()
                .ok_or(anyhow!(format!("Expected bool: {}", key)))
                .ok()
        })
        .unwrap_or(None))
}
