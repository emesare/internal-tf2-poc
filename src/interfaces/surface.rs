use std::os::raw::c_char;

// TODO: research CFontManager more.
// TODO: research FontDrawType.
// TODO: personally dislike using surface, switch to directx 9 (windows_sys::Win32::Graphics)

#[repr(C)]
enum FontDrawType {
    // Use the "additive" value from the scheme file
    FONT_DRAW_DEFAULT = 0,

    // Overrides
    FONT_DRAW_NONADDITIVE = 1,
    // FONT_DRAW_ADDITIVE = 1,
    FONT_DRAW_TYPE_COUNT = 2,
}

pub type DrawSetColorFn =
    unsafe extern "thiscall" fn(this: *const Surface, r: u8, g: u8, b: u8, a: u8);
pub type DrawRectLineFn =
    unsafe extern "thiscall" fn(this: *const Surface, x0: i32, y0: i32, x1: i32, y1: i32);
pub type DrawSetTextColorFn =
    unsafe extern "thiscall" fn(this: *const Surface, r: u8, g: u8, b: u8, a: u8);
pub type DrawSetTextPosFn = unsafe extern "thiscall" fn(this: *const Surface, x: i32, y: i32);
pub type DrawSetTextFontFn = unsafe extern "thiscall" fn(this: *const Surface, font: usize);
pub type DrawPrintTextFn =
    unsafe extern "thiscall" fn(this: *const Surface, *const u16, i32, FontDrawType);

#[repr(C)]
#[derive(Debug)]
pub struct Surface {
    pub vtable: usize,
}

// Just following the naming convention set forth by microsoft when it comes to ffi (see windows-rs)
#[allow(non_snake_case)]
impl Surface {
    pub unsafe fn DrawSetColor(&self, r: u8, g: u8, b: u8, a: u8) {
        let set_draw_color = std::mem::transmute::<_, DrawSetColorFn>(
            (self.vtable as *mut usize).offset(11).read() as *mut usize,
        );

        set_draw_color(self as *const Self, r, g, b, a);
    }

    pub unsafe fn DrawFilledRect(&self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let draw_filled_rect = std::mem::transmute::<_, DrawRectLineFn>(
            (self.vtable as *mut usize).offset(12).read() as *mut usize,
        );

        draw_filled_rect(self as *const Self, x0, y0, x1, y1);
    }

    pub unsafe fn DrawOutlinedRect(&self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let draw_outlined_rect = std::mem::transmute::<_, DrawRectLineFn>(
            (self.vtable as *mut usize).offset(14).read() as *mut usize,
        );

        draw_outlined_rect(self as *const Self, x0, y0, x1, y1);
    }

    pub unsafe fn DrawSetTextColor(&self, r: u8, g: u8, b: u8, a: u8) {
        let draw_set_text_color = std::mem::transmute::<_, DrawSetTextColorFn>(
            (self.vtable as *mut usize).offset(19).read() as *mut usize,
        );

        draw_set_text_color(self as *const Self, r, g, b, a);
    }

    pub unsafe fn DrawSetTextPos(&self, x: i32, y: i32) {
        let draw_set_text_pos = std::mem::transmute::<_, DrawSetTextPosFn>(
            (self.vtable as *mut usize).offset(20).read() as *mut usize,
        );

        draw_set_text_pos(self as *const Self, x, y);
    }

    pub unsafe fn DrawSetTextFont(&self, font: usize) {
        let draw_set_text_pos = std::mem::transmute::<_, DrawSetTextFontFn>(
            (self.vtable as *mut usize).offset(17).read() as *mut usize,
        );

        draw_set_text_pos(self as *const Self, font);
    }

    pub unsafe fn DrawPrintText(&self, text: *const u16, len: i32) {
        let print_text = std::mem::transmute::<_, DrawPrintTextFn>(
            (self.vtable as *mut usize).offset(22).read() as *mut usize,
        );

        print_text(
            self as *const Self,
            text,
            len,
            FontDrawType::FONT_DRAW_DEFAULT,
        );
    }

    // Wrapper around SetTextPos, PrintText
    pub unsafe fn DrawText(&self, text: &str, x: i32, y: i32) {
        self.DrawSetTextPos(x, y);
        let strbuf: Vec<u16> = text.encode_utf16().collect();
        self.DrawPrintText(strbuf.as_ptr(), strbuf.len().try_into().unwrap());
    }

    pub unsafe fn AddFont(name: &str, size: i32, weight: i32) {}
}

unsafe impl Send for Surface {}
