use clipboard_win::Clipboard;
use clipboard_win::formats;

use crate::Windows::formats;
use crate::Windows::ClipBoard;

pub mod ClipBoard{
    pub fn get_it() -> String{
        let clip_text= Clipboard::get_it();

        clip_text
    }
}
