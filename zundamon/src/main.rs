mod windows;
mod voicevox_request;
mod fileing;

fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let text= windows::ClipBoard::get_it();
    
    let do_file= fileing::SaveTxtFile::init(text);
    
    if do_file.is_ok(){
        let do_request= voicevox_request::Save::json_init();
        voicevox_request::Save::json_init();
    }

    Ok(())
}
