use std::env;
use std::fs::File;
use std::io::{Write, BufReader};
use csv::ReaderBuilder;
use reqwest::header::AUTHORIZATION;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No CSV file provided".into());
    }

    // Open the csv file
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(reader);

    let openai_api_key = env::var("OPENAI_API_KEY")?;

    let client = reqwest::Client::new();

    for result in csv_reader.records() {
        let record = result?;
        let chunk = record.get(0).unwrap_or("");
        let res = client.post("https://api.openai.com/v1/embeddings")
            .header(AUTHORIZATION, format!("Bearer {}", openai_api_key))
            .json(&json!({
                "input": chunk,
                "model": "text-embedding-ada-002"
            }))
            .send()
            .await?;
        let text = res.text().await?;

        // Write the response to a file
        let mut file = File::create("embedding_vector.txt")?;
        file.write_all(text.as_bytes())?;
    }
    Ok(())
}
