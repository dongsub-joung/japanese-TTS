mod Windows;
mod File;
mod VoicevoxRequest;

fn main() {
    let text= Windows::GetClipBoardInfo::init();
    
    File::SaveTxtFile::init(text);
    
    let json= VoicevoxRequest::Save::jsonTnit();;
    if json == Ok(()){
        let audio= VoicevoxRequest::Save::audioInit();
        if audio == Ok(()) {
            println!("zundamon!")
        }else{
            println!("Faild making a audio");
        }
    }else{
        println!("Faild making a JSON");
    }
}
