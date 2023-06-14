use std::{ffi, intrinsics::transmute, os::raw::c_char};

use log::debug;
use windows::Win32::{
    Foundation::{FARPROC, HINSTANCE, PSTR},
    System::LibraryLoader::GetProcAddress,
};

use crate::{c_str, win_str_ptr};

pub mod debug_overlay;
pub mod engine_client;
pub mod entity_list;
pub mod panel;
pub mod surface;

pub fn get_interface<T: Sized>(module: HINSTANCE, name: &str) -> &'static T {
    let function_ptr: FARPROC =
        unsafe { GetProcAddress(module, PSTR(win_str_ptr!("CreateInterface"))) };

    let function = unsafe {
        transmute::<_, unsafe extern "C" fn(name: *const c_char, return_code: i8) -> *mut usize>(
            function_ptr.unwrap(),
        )
    };

    let interface = unsafe { function(c_str!("{}", name).as_ptr() as *const c_char, 0) };

    if interface.is_null() {
        log::error!("interface {} is null", name);
    }

    // use return address to get vtable and map out all the virtual methods (index * 4 bytes).
    debug!("interface {}: {:?}", name, interface);

    unsafe { std::mem::transmute::<_, &'static T>(interface) }
}
