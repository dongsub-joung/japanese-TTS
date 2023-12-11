# Japanese-TTS
in windows written rust  
play a sound using clipboard data

## dependency - installed  
https://voicevox.hiroshiba.jp/  
https://cmake.org/download/

## setting  
1. 

> administrator mode  
Pop up the voicevox.exe as windows administrator mode

2.  

> CORS Policy Mode -all

http://localhost:50021/setting

3. Change PAHT valuable `main.rs`  

```
input your path
const PATH: &str= "C:/Users/kiririn/git/japanese-TTS/zundamon/audio.wav";
```    

## Usage
```
cd zundamon
cargo run
```
