use std::os::raw::c_char;

pub type GetNameFn =
    unsafe extern "thiscall" fn(this: *const Panel, panel_id: u32) -> *const c_char;

#[repr(C)]
#[derive(Debug)]
pub struct Panel {
    pub vtable: usize,
}

// 10001220  68bc790410         push    data_100479bc {var_4}  {"VGUI_Panel009"}
// 10001225  68b08c0110         push    sub_10018cb0 {var_8} <--- allocate memory for vtable
// 1000122a  b9d8af0510         mov     ecx, 0x1005afd8
// 1000122f  e86cad0100         call    sub_1001bfa0
// 10001234  c3                 retn     {__return_addr}

// Panel \'%s/%s\' has invalid client: %p.\n

// Just following the naming convention set forth by microsoft when it comes to ffi (see windows-rs)
#[allow(non_snake_case)]
impl Panel {
    pub unsafe fn GetName(&self, panel_id: u32) -> *const c_char {
        let get_name = std::mem::transmute::<_, GetNameFn>(
            (self.vtable as *mut usize).offset(36).read() as *mut usize,
        );

        get_name(self as *const Self, panel_id)
    }
}

unsafe impl Send for Panel {}
