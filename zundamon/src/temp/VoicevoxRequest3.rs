use voicevox_client::Client;
use reqwest::Result;
use std::{io::Write, fs::File};

pub mod Save {
    use super::*;

    pub async fn init(txt: String) -> Result<()> {
        let client = Client::new("http://localhost:50021".to_string());
        let audio_query = client
            .create_audio_query(txt.as_str(), 1, None)
            .await?;
        let mut audio = audio_query.synthesis(1).await?;
        
        // let mut file = File::create("./audio.wav").unwrap();
        // let mut file = File::create("C:/Users/kiririn/git/japanese-TTS/zundamon/audio.wav").unwrap();
        // file.write_all(&audio).unwrap();
        
        let output_file = File::create("audio.wav");
        let _= output_file.unwrap().write_all(&mut audio);

        Ok(())
    }
}