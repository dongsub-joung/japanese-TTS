mod Windows;
mod File;
mod VoicevoxRequest;

fn main() {
    let text= Windows::ClipBoard::get_it();
    
    File::SaveTxtFile::init(text);
    
    let json= VoicevoxRequest::Save::jsonTnit();
    if json.is_ok(){
        let audio= VoicevoxRequest::Save::audioInit();
        if audio.is_ok(){
            println!("zundamon!")
        }else{
            println!("Faild making a audio");
        }
    }else{
        println!("Faild making a JSON");
    }
}
