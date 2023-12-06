use reqwest::Error;

pub mod SaveJson {
    // Function to create a query for audio
    async fn create_query(input_text: &str, selected_character: &str) -> Result<serde_json::Value, Error> {
        let base_url = "https://localhost:50021"; // Replace with your base URL
        let api_url = "audio_query";
        let api_endpoint = format!("{}/{}?text={}&speaker={}", base_url, api_url, input_text, selected_character);
    
        let client = reqwest::Client::new();
        let response = client
            .post(&api_endpoint)
            .header("Content-Type", "application/json")
            .send()
            .await?;

    
        let data = response.json().await?;
        Ok(data)
    }
    
    #[tokio::main]
    pub async fn init() {
        // Example usage
        let input_text = "解けた！を世界に届けたい"; // Replace with your input text
        let selected_character = "1"; // Replace with your selected character
    
        match create_query(input_text, selected_character).await {
            Ok(data) => {
                println!("Response: {:?}", data);
                // Handle the returned data as needed
            }
            Err(e) => eprintln!("Error creating query: {}", e),
        }
    }    
}
