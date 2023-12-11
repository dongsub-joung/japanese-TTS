use voicevox::{*, client::VVClient};

fn main() {
    let vvc= VVClient::new();
    let qs= vvc.query(txt, id);
    let dat = vvc.synth(qs, id).unwrap();
    vvc.speak(dat, 3).unwrap();
}
