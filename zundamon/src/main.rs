mod Windows;
mod VoicevoxRequest;
mod Fileing;

use std::fs::File;
use std::io::Read;

use reqwest::Error;


fn main() -> Result<(), Error>{
    let text= Windows::ClipBoard::get_it();
    
    let _= Fileing::SaveTxtFile::init(text);
    
    let file = std::fs::File::open("text.txt");
    let mut text = String::new();
    let _= file.unwrap().read_to_string(&mut text);

    let _= VoicevoxRequest::Save::init(text);

    Ok(())
}
