use reqwest::blocking::get;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.waifu.im/search?is_nsfw=true";
    let response = get(url)?;

    if response.status().is_success() {
        let body = response.text()?;

        let parsed: Value = serde_json::from_str(&body)?;
        let pretty_json = serde_json::to_string_pretty(&parsed)?;

        println!("Pretty JSON:\n{}", pretty_json);
    } else {
        eprintln!("Failed to fetch URL: {}", response.status());
    }

    Ok(())
}
