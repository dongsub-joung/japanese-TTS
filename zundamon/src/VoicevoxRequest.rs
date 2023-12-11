use std::fs::File;
use std::io::Read;
use reqwest;

use crate::reqwest::blocking::Client;

pub mod Save {
    pub fn jsonTnit() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
    
        // Read the contents of text.txt file
        let mut file = File::open("text.txt")?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
    
        let url = "http://localhost:50021/audio_query";
        let params = [("speaker", "1")];
        let mut response = client.post(url)
            .form(&params)
            .body(text)
            .send()?;
    
        let mut output_file = File::create("query.json")?;
        response.copy_to(&mut output_file)?;
    
        Ok(())
    }
    
    pub fn audioInit() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
    
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
