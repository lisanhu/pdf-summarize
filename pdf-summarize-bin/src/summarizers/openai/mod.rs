use std::time::Duration;

use crate::summarizers::{Summarize, Error};
use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION, HeaderValue};
use serde_json::json;

pub struct GPT_3_5_TURBO_Summarizer {
    api_key: String,
}

impl GPT_3_5_TURBO_Summarizer {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl Summarize for GPT_3_5_TURBO_Summarizer {
    fn summarize(&self, text: &str) -> Result<String, Error> {
        let url = "https://api.openai.com/v1/chat/completions";

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap());

        // let body = format!(r#"{{ "model": "gpt-3.5-turbo", "messages": [{{"role": "system", "content": "You are a helpful assistant that helps the user to read articles and papers. The user will provide the text content to you and please summarize what the authors write in it. Include necessary technique details if possible. Do not include comments about its content."}}, {{"role": "user", "content": "Please summarize this text: {}"}}] }}"#, text);
        let body = json!(
            {
                "model": "gpt-3.5-turbo-16k",
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a helpful assistant that helps the user to read articles and papers. The user will provide the text content to you and please summarize what the authors write in it. Include necessary technique details if possible. Do not include comments about its content."
                    },
                    {
                        "role": "user",
                        "content": format!("Please summarize this text: {}", text)
                    }
                ]
            }
        );

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(url)
            .headers(headers)
            .json(&body)
            .timeout(Duration::from_secs(60))
            .send()
            .unwrap()
            .text()
            .unwrap();

        // eprintln!("{}", res);
        let res = serde_json::from_str::<serde_json::Value>(&res).unwrap();
        let input_tokens = res["usage"]["prompt_tokens"].as_u64().unwrap();
        let output_tokens = res["usage"]["completion_tokens"].as_u64().unwrap();
        eprintln!("cost: input {} tokens, ", input_tokens);
        eprintln!("cost: output {} tokens, ", output_tokens);
        eprintln!("cost: ${}, ", input_tokens as f32 * 0.003 / 1000f32 + output_tokens as f32 * 0.004 / 1000f32);
        return Ok(res["choices"][0]["message"]["content"].as_str().unwrap().to_string());

    }
}
