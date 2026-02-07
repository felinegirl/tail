#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

use crate::sourcepp::vpkpp::bindings::vpkpp_pack_file_handle_t;

pub mod bindings;

fn to_stupid_strings(basestring: &str) -> *const i8 {
    let form = CString::new(basestring).unwrap();
    form.as_ptr() as *const i8
}

pub(crate) fn openvpk(path: &str) -> i32{
    unsafe {
        let mut fuckyou: vpkpp_pack_file_handle_t;
        fuckyou = bindings::vpkpp_open(to_stupid_strings(path), None, None);
            
        let awa = bindings::vpkpp_vpk_get_version(fuckyou);
        
        dbg!(fuckyou)
        // for a in 0..1 {

        // }
        
    };
    
    1
}


