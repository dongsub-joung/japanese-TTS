use clipboard_win::{Clipboard, formats, Getter, Setter};

use std::fs::File;
use std::io::Write;

pub mod GetClipBoardInfo{

    pub fn init() -> String{
        let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
        formats::Unicode.write_clipboard(&SAMPLE).expect("Write sample");

        let mut output = String::new();

        let clip_text= formats::Unicode.read_clipboard(&mut output).expect("Read sample");
   
        // output.clear();
        
        clip_text
    }
}

pub mod SaveTxtFile{
    pub fn init(text_content: String){
  
    // Open or create a file (in this case named "output.txt")
    let mut file = File::create("text.txt")?;

    // Write the text content to the file
    file.write_all(text_content.as_bytes())?;

    println!("Text saved to 'output.txt' file.");

    Ok(())
    }
}