use clipboard_win::{Clipboard, formats, Getter, Setter};

pub mod ClipBoard{
    pub fn get_it() -> String{
        let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
        formats::Unicode.write_clipboard(&SAMPLE).expect("Write sample");

        let mut output = String::new();

        let clip_text= formats::Unicode.read_clipboard(&mut output).expect("Read sample");
   
        // output.clear();
        
        clip_text
    }
}
