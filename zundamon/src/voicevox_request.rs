use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use serde_urlencoded;
use reqwest::header::{CONTENT_TYPE, CONNECTION};

pub mod save {
    use super::*;
    use std::fmt::format;

    const BASE: &str= "http://localhost:50021";
    const SPEAKER: &str= "1";

    fn open_then_read()-> Result<(), Box<dyn std::error::Error>> {
        let mut text = String::new();
        
        let mut file = std::fs::File::open("text.txt")?;
        file.read_to_string(&mut text)
            .expect("{can't read a text file}");

        Ok(())
    }

    fn set_hashmap_value() -> HashMap<&str, &str>{
        vec![("text", text.as_str()), ("speaker", "1")].into_iter().collect()
    }

    pub fn json_init() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        
        open_then_read();
    
        // let url = "http://localhost:50021/audio_query";
        let url= format!("{}/audio_query", BASE);
        let m: HashMap<&str, &str>= set_hashmap_value();
        let params= serde_urlencoded::to_string(m)?;
        
        let mut response = client
            .post(format!("{}?{}", url, params)).send()?;
    
        let mut output_file = File::create("query.json")?;
        response.copy_to(&mut output_file)?;
    
        Ok(())
    }
    
    pub fn audio_init() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
    
        // let url = "http://localhost:50021/synthesis?speaker=1";
        let url= format!("{}/synthesis?speaker={}", BASE, SPEAKER);

        let mut file = File::open("query.json")?;
        let mut json_content = String::new();
        file.read_to_string(&mut json_content)?;
    
        let mut response = client.post(url)
            .header(CONTENT_TYPE, "application/json")
            .header(CONNECTION, "close")
            .body(json_content)
            .send()?;
    
        let mut output_file = File::create("audio.wav")?;
        response.copy_to(&mut output_file)?;
    
        Ok(())
    }
    
}