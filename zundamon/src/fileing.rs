use std::fs::File;
use std::io::Write;

pub mod save_txtfile{
    use super::*;
    const FILE_NAME: &str= "text.txt";

    pub fn init(text_content: String) -> Result<(), Box<dyn std::error::Error>>{
    
    let mut file = File::create(FILE_NAME)?;

    file.write_all(text_content.as_bytes()).expect("text file io err");

    println!("Text saved to 'output.txt' file.");

    Ok(())
    }
}