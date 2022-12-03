#![no_main]
use libfuzzer_sys::fuzz_target;
use unicode_width::UnicodeWidthStr;
use std::str;

fuzz_target!(|data: &[u8]| {
    if data.len() > 1 {
        let opt = data[0];
        match str::from_utf8(&data[1..]) {
            Ok(in_string)=>{
                match opt {
                    0=>{UnicodeWidthStr::width(in_string);},
                    1=>{UnicodeWidthStr::width_cjk(in_string);},
                    _=>()
                }
            },
            Err(..)=>()
        }
    }
});
