use std::os::raw::c_char;

use crate::math::Vec3;

pub type ScreenPositionFn = unsafe extern "thiscall" fn(
    this: *const DebugOverlay,
    point: *mut Vec3,
    screen: *mut Vec3,
) -> i32;

#[repr(C)]
#[derive(Debug)]
pub struct DebugOverlay {
    pub vtable: usize,
}

// Just following the naming convention set forth by microsoft when it comes to ffi (see windows-rs)
#[allow(non_snake_case)]
impl DebugOverlay {
    // TODO: Based on the disasm it returns a 1 if success

    pub unsafe fn ScreenPosition(&self, point: *mut Vec3, screen: *mut Vec3) -> i32 {
        let screen_position = std::mem::transmute::<_, ScreenPositionFn>(
            (self.vtable as *mut usize).offset(10).read() as *mut usize,
        );

        screen_position(self as *const Self, point, screen)
    }
}

unsafe impl Send for DebugOverlay {}
