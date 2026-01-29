use crate::error::{Error, Result};
use crate::service::CONTEXT;
use reqwest::Client;
use serde_json::Value;

lazy_static! {
    static ref HTTP_CLIENT: Client = Client::new();
}

fn deepseek_api_key() -> Result<String> {
    if let Ok(value) = std::env::var("DEEPSEEK_API_KEY") {
        if !value.trim().is_empty() {
            return Ok(value);
        }
    }

    if !CONTEXT.config.deepseek_api_key.trim().is_empty() {
        return Ok(CONTEXT.config.deepseek_api_key.clone());
    }

    Err(Error::from(
        "DeepSeek API key missing; set DEEPSEEK_API_KEY or deepseek_api_key",
    ))
}

fn deepseek_base_url() -> Result<String> {
    let base_url = CONTEXT.config.deepseek_api_base_url.trim();
    if base_url.is_empty() {
        return Err(Error::from("DeepSeek API base URL is empty"));
    }

    Ok(base_url.trim_end_matches('/').to_string())
}

fn deepseek_default_model() -> String {
    let model = CONTEXT.config.deepseek_default_model.trim();
    if model.is_empty() {
        "deepseek-chat".to_string()
    } else {
        model.to_string()
    }
}

pub async fn chat(mut payload: Value) -> Result<Value> {
    let body = payload
        .as_object_mut()
        .ok_or_else(|| Error::from("Request body must be a JSON object"))?;

    if !body.contains_key("model") {
        body.insert(
            "model".to_string(),
            Value::String(deepseek_default_model()),
        );
    }

    let api_key = deepseek_api_key()?;
    let url = format!("{}/chat/completions", deepseek_base_url()?);

    let resp = HTTP_CLIENT
        .post(&url)
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|err| Error::from(err.to_string()))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|err| Error::from(err.to_string()))?;

    if !status.is_success() {
        return Err(Error::from(format!(
            "DeepSeek API error ({}): {}",
            status.as_u16(),
            text
        )));
    }

    serde_json::from_str(&text).map_err(|err| Error::from(err.to_string()))
}
