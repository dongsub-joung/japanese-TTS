extern crate winapi;

use std::ptr;
use std::ffi::CString;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use winapi::um::winuser::{OpenClipboard, GetClipboardData, CloseClipboard, CF_TEXT};
use winapi::um::handleapi::GlobalLock;
use winapi::um::memoryapi::GlobalUnlock;
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::shared::minwindef::{HGLOBAL, LPVOID};

use std::fs::File;
use std::io::Write;

pub mod GetClipBoardInfo{
    pub fn init() -> String{
        let mut result_text;
    
        unsafe {
            if OpenClipboard(ptr::null_mut()) != 0 {
                let h_clipboard_data = GetClipboardData(CF_TEXT);
    
                if !h_clipboard_data.is_null() {
                    let text_ptr = GlobalLock(h_clipboard_data) as *mut i8;
                    if !text_ptr.is_null() {
                        let text = CString::from_raw(text_ptr).into_string().unwrap();
                        
                        // println!("Clipboard Data: {}", text);
                        result_text= text;
                        GlobalUnlock(h_clipboard_data);
                    } else {
                        println!("Failed to access clipboard data.");
                    }
                } else {
                    println!("No text data in clipboard.");
                }
                CloseClipboard();
            } else {
                println!("Failed to open clipboard. Error code: {}", GetLastError());
            }
        }
    
        result_text
    }
}

pub mod SaveTxtFile{
    pub fn init(text_content: String){
  
    // Open or create a file (in this case named "output.txt")
    let mut file = File::create("text.txt")?;

    // Write the text content to the file
    file.write_all(text_content.as_bytes())?;

    println!("Text saved to 'output.txt' file.");

    Ok(())
    }
}