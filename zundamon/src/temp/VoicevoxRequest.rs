// voicevox
extern crate reqwest;
use reqwest::Error;

// file
use std::io::Write;
use std::fs::File;
use std::io::Result;

pub mod SaveingFile{
    fn saveAResponseFile(content: String) -> Result<()> {
        // Text content to be saved in the file
        // let content = "Hello, this is some content to be saved in a file.";
    
        // Create or open a file in write mode (creates a new file or truncates an existing file)
        let mut file = File::create("response.json")?;

        // Write the content to the file
        file.write_all(content.as_bytes())?;
    
        println!("File saved successfully.");
    
        Ok(())
    }
    
    #[tokio::main]
    pub async fn sendTheRequestToAPI(data: String) -> Result<(), Error> {
        // Voicevox API endpoint URL
        let url = "http://localhost:50021";
    
        // Your input data
        // let data = r#"{"text": "Hello, this is a test."}"#;
        let data= "ユーザー登録";
    
        // Make a POST request to the Voicevox API
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .body(data)
            .send()
            .await?;
    
        // Check if the request was successful
        if response.status().is_success() {
            let body = response.text().await?;
            
            // println!("Response body: {}", body);
            saveAResponseFile(body);
        } else {
            println!("Request failed with status code: {}", response.status());
        }
    
        Ok(())
    }
}
