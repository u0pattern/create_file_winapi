extern crate kernel32;
extern crate winapi;

use std::ffi::CString;
use kernel32::CreateFileA;
use winapi::um::winnt::{GENERIC_READ, FILE_SHARE_READ,FILE_ATTRIBUTE_NORMAL};
use winapi::um::fileapi::CREATE_NEW;

fn main() {
	let file_name = CString::new("C:\\Users\\DummyUser\\Desktop\\test.txt").unwrap();
	
    unsafe {
        CreateFileA(
			file_name.as_ptr(),
			GENERIC_READ,
			FILE_SHARE_READ,
            		std::ptr::null_mut(),
			CREATE_NEW,
			FILE_ATTRIBUTE_NORMAL,
			std::ptr::null_mut()
        );
    }
}
