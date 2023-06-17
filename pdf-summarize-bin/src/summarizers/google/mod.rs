use crate::summarizers::{Summarize, Error};
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde_json::json;

pub struct Palm2Summarizer {
    api_key: String,
}

impl Palm2Summarizer {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl Summarize for Palm2Summarizer {
    fn summarize(&self, text: &str) -> Result<String, Error> {
        let url = format!("https://generativelanguage.googleapis.com/v1beta2/models/chat-bison-001:generateMessage?key={}", self.api_key);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let body = json!({
            "prompt": {
                "context": "You are a helpful assistant that helps the user to read articles and papers. The user will provide the text content to you and please summarize what the authors write in it. Include necessary technique details if possible. Do not include comments about its content.",
                "examples": [],
                "messages": [{
                    "author": "user",
                    "content": format!("Please summarize this text: {}", text)
                }]
            },
            "temperature": 0.25,
            "top_k": 40,
            "top_p": 0.95,
            "candidate_count": 1
        });

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .unwrap()
            .text()
            .unwrap();

        // eprintln!("{}", res);
        let mut res = serde_json::from_str::<serde_json::Value>(&res).unwrap();
        if let Some(obj) = res.as_object_mut() {
            obj.remove("messages");
        }
        // eprintln!("{}", res);

        if let Some(filters) = res.get("filters") {
            // The "candidates" field exists
            return Err(Error{
                message: format!("Response blocked: {}", filters.to_string())
            });
        } else {
            // The "candidates" field does not exist
            let res = res["candidates"][0]["content"].as_str().unwrap();
            return Ok(res.to_owned());
        }

    }
}
