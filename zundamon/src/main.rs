mod Windows;
mod VoicevoxRequest;
mod VoicevoxResponse;

fn main() {
    let text= Windows::GetClipBoardInfo::init();

    VoicevoxRequest::SaveJSON::init();
    // VoicevoxResponse::SendAudio::init()
}
