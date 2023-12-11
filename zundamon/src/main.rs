mod windows;
mod voicevox_request;
mod fileing;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use soloud::*;

fn init() ->  Result<(), Box<dyn std::error::Error>> {
    let text= windows::clip_board::get_it();
    
    let do_file= fileing::save_txtfile::init(text);
    
    if do_file.is_ok(){
        let do_request= voicevox_request::save::json_init();
        if do_request.is_ok() {
            let _= voicevox_request::save::audio_init();
        }
    }

    Ok(())
}


fn play_audio(pwd: &str){
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    let load_work= wav.load(pwd);
    if load_work.is_ok(){
        sl.play(&wav);
    }else{
        println!("Sound load is not work");
    }
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn main() {
    const PATH: &str= "C:/Users/kiririn/git/japanese-TTS/zundamon/audio.wav";
    
    // A flag to indicate when to stop the loop
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Handling Ctrl+C signal
    ctrlc::set_handler(move || {
        println!("\nCtrl+C pressed. Shutting down...");
        running_clone.store(false, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl+C handler");

    // Main loop
    while running.load(Ordering::Relaxed) {
        // Your code here - This loop will keep running until Ctrl+C is pressed
        println!("Running...");

        thread::sleep(Duration::from_secs(5));

        let _= init();
        play_audio(PATH);

        // Simulating some work
        thread::sleep(Duration::from_secs(10));
    }

    println!("Goodbye!");
}