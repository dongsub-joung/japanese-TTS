use std::fs::File;
use reqwest::blocking::Client;
use std::io::Read;

pub mod SaveJson {
    fn init() -> Result<(), Box<dyn std::error::Error>> {
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
    
}
