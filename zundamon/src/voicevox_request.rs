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

    fn open_then_read(file_name: &str)-> Result<String, Box<dyn std::error::Error>> {
        let mut text = String::new();
        
        let mut file = std::fs::File::open(file_name)?;
        file.read_to_string(&mut text)
            .expect("{can't read a text file}");

        Ok(text)
    }

    pub fn json_init() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        
        let body= open_then_read("text.txt").unwrap();

        // let url = "http://localhost:50021/audio_query";
        let url= format!("{}/audio_query", BASE);
        let m: HashMap<&str, &str>= 
            vec![("text", body.as_str()), ("speaker", "1")].into_iter().collect();
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

        let json_content= open_then_read("query.json").unwrap();
    
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