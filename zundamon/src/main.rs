mod windows;
mod voicevox_request;
mod fileing;

use std::fs::File;
use std::io::Read;

use reqwest::Error;


fn main() -> Result<(), Error>{
    let text= windows::ClipBoard::get_it();
    
    let do_file= fileing::SaveTxtFile::init(text);
    
    if do_file.is_ok(){
        let do_request= voicevox_request::Save::json_init();
        if do_request.is_ok() {
            voicevox_request::Save::json_init();
        }
    }

    Ok(())
}
