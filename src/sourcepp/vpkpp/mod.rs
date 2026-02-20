#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi::{CStr, CString, c_char}, str::Utf8Error};

use crate::sourcepp::vpkpp::bindings::vpkpp_pack_file_handle_t;

pub mod bindings;

//converts cool rust strings to stupid c "strings"
fn stupid(bwa: &str) -> *const c_char{
    bwa.as_ptr() as *const c_char
}

//converts stupid c strings to cool rust strings
fn cool(bwa: *const c_char) -> Result<&'static str, Utf8Error>{
    unsafe { CStr::from_ptr(bwa) }.to_str()
}

//converts sourcepp_string_t to rust string
fn death(dumb: bindings::sourcepp_string_t) -> String{
    unsafe { String::from_raw_parts(dumb.data,dumb.size as usize, dumb.size as usize) }
}

pub(crate) fn openvpk(path: &str) -> i32{
    unsafe {

        let path: &str = &format!("{path}\0");

        let fuckyou: vpkpp_pack_file_handle_t;
        fuckyou = bindings::vpkpp_open(stupid(path), None, None);
            
        let awa = bindings::vpkpp_get_filename(fuckyou);
        dbg!(death(awa));
        
        let awa = bindings::vpkpp_get_entry_count(fuckyou, 0);
        dbg!(awa);
        
    };
    
    1
}


