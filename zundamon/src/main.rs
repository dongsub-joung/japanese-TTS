mod windows;
mod voicevox_request;
mod fileing;

fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let text= windows::clipBoard::get_it();
    
    let do_file= fileing::save_txtfile::init(text);
    
    if do_file.is_ok(){
        let do_request= voicevox_request::save::json_init();
        if do_request.is_ok() {
            let _= voicevox_request::save::audio_init();
        }
    }

    Ok(())
}
