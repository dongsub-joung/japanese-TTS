mod Windows;
mod VoicevoxRequestJson;
mod VoicevoxRequestAudio;

fn main() {
    let text= Windows::GetClipBoardInfo::init();
    Windows::SaveTxtFile::init(text);
    VoicevoxRequestJson::SaveJSON::init();
    VoicevoxRequestAudio::SendAudio::init();
}
