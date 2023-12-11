use std::fs::File;
use std::io::Read;
use reqwest::{Client, Error};
use std::collections::HashMap;
use serde_urlencoded;

pub mod Save {
    use std::fmt::format;

    use super::*;
    const BASE_URL: &str= "http://localhost:50021";

    pub fn json_init() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        
        // Read the contents of text.txt file
        let mut file = std::fs::File::open("text.txt")?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
    
        let url = "http://localhost:50021/audio_query";

        let m: HashMap<&str, &str>= vec![
            ("text", text.as_str()), ("speaker", "1")].into_iter().collect();

        let params= serde_urlencoded::to_string(m)?;
        let mut response = client
            .post(format!("{}?{}", url, params)).send()?;
    
        let mut output_file = File::create("query.json")?;
        response.copy_to(&mut output_file)?;
    
        Ok(())
    }
    
    pub fn audio_init() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
    
        let url = "http://localhost:50021/synthesis?speaker=1";
        let mut file = File::open("query.json")?;
        let mut json_content = String::new();
        file.read_to_string(&mut json_content)?;
    
        let mut response = client.post(url)
            .header("Content-Type", "application/json")
            .body(json_content)
            .send()?;
    
        let mut output_file = File::create("audio.wav")?;
        response.copy_to(&mut output_file)?;
    
        Ok(())
    }
    
}