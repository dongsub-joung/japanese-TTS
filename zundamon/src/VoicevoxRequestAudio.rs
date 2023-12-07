use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;

pub mod SaveAudio {
    fn init() -> Result<(), Box<dyn std::error::Error>> {
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
