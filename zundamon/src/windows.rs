pub mod clip_board{
    pub fn get_it() -> String{
        let clip_text= clipboard_win::get_clipboard_string()
            .expect("clipboard io err");

        clip_text
    }
}
