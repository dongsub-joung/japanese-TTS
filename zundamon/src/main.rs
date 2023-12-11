mod Windows;
mod VoicevoxRequest;
mod Fileing;

use std::fs::File;
use std::io::Read;

use reqwest::Error;


fn main() -> Result<(), Error>{
    let text= Windows::ClipBoard::get_it();
    
    let do_file= Fileing::SaveTxtFile::init(text);
    
    if do_file.is_ok(){
        let do_request= VoicevoxRequest::Save::json_init();
        if do_request.is_ok() {
            VoicevoxRequest::Save::json_init();
        }
    }

    Ok(())
}
