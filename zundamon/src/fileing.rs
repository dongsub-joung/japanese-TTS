use std::fs::File;
use std::io::Write;

pub mod SaveTxtFile{
    use super::*;
    
    pub fn init(text_content: String) -> Result<(), Box<dyn std::error::Error>>{
  
    // Open or create a file (in this case named "output.txt")
    let mut file = File::create("text.txt")?;

    // Write the text content to the file
    file.write_all(text_content.as_bytes())?;

    println!("Text saved to 'output.txt' file.");

    Ok(())
    }
}