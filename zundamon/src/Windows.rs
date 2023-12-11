use clipboard_win::Clipboard;

pub mod ClipBoard{
    use super::*;
    pub fn get_it() -> String{
        let clip_text= clipboard_win::get_clipboard_string().unwrap();
        
        clip_text
    }
}
