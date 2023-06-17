#![allow(non_camel_case_types)]
#![allow(dead_code)]

use pdf_extract::extract_text;
mod summarizers;
use crate::summarizers::{google::Palm2Summarizer, Summarize, openai::GPT_3_5_TURBO_Summarizer};
use std::time::Instant;
use dotenv::dotenv;
use std::env;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Please provide a file path as an argument");

    let text = extract_text(file_path)?;

    // let summarizer = Palm2Summarizer::new(env::var("GOOGLE_API_KEY").unwrap());
    let summarizer = GPT_3_5_TURBO_Summarizer::new(env::var("OPENAI_API_KEY").unwrap());
    let start = Instant::now();
    println!("{}", summarizer.summarize(&text).unwrap());
    let elapsed = start.elapsed();
    eprintln!("Elapsed time: {:.2?}", elapsed);

    Ok(())
}
