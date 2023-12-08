mod Windows;
mod VoicevoxRequestJson;
mod VoicevoxRequestAudio;

fn main() {
    let text= Windows::GetClipBoardInfo::init();
    Windows::SaveTxtFile::init(text);
    
    let json= VoicevoxRequestJson::SaveJSON::init();;
    if json == Ok(()){
        let audio= VoicevoxRequestAudio::SendAudio::init();
        if audio == Ok(()) {
            println!("zundamon!")
        }else{
            println!("Faild making a audio");
        }
    }else{
        println!("Faild making a JSON");
    }
}
