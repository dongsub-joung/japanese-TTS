use reqwest::Error;

pub mod SaveAudio {

    // Function to create a voice using the API
    async fn create_voice(
        query_json: serde_json::Value,
        selected_character: &str,
    ) -> Result<Vec<u8>, Error> {
        let base_url = "https://localhost:50021";; // Replace with your base URL
        let api_url = "synthesis";
        let api_endpoint = format!("{}/{}?speaker={}", base_url, api_url, selected_character);

        let client = reqwest::Client::new();
        let response = client
            .post(api_endpoint)
            .header("Content-Type", "application/json")
            .json(&query_json)
            .send()
            .await?;

        let data = response.bytes().await?.to_vec();
        Ok(data)
    }

    #[tokio::main]
    pub async fn init() {
        // Example usage
        let query_json = serde_json::json!({"key": "value"}); // Replace with your JSON payload
        let selected_character = "1"; // Replace with your selected character

        match create_voice(query_json, selected_character).await {
            Ok(data) => {
                println!("Response: {:?}", data);
                // Handle the returned data as needed
            }
            Err(e) => eprintln!("Error creating voice: {}", e),
        }
    }
}
