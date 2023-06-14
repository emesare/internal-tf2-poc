use std::os::raw::c_char;

use crate::math::Vec3;

// Temp location
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PlayerInfo {
    pad: [u8; 0x8],
    pub name: *const c_char, // max len of 32 according to source engine
}

pub type GetScreenSizeFn =
    unsafe extern "thiscall" fn(this: *const EngineClient, width: *mut i32, height: *mut i32);
pub type GetPlayerInfoFn =
    unsafe extern "thiscall" fn(this: *const EngineClient, player_info: *mut PlayerInfo);
pub type IsInGameFn = unsafe extern "thiscall" fn(*const EngineClient) -> bool;
pub type GetMaxClientsFn = unsafe extern "thiscall" fn(*const EngineClient) -> i32;
pub type GetViewAnglesFn = unsafe extern "thiscall" fn(*const EngineClient, *mut Vec3);
pub type ExecuteClientCmdFn =
    unsafe extern "thiscall" fn(this: *const EngineClient, command: *const c_char);

#[repr(C)]
#[derive(Debug)]
pub struct EngineClient {
    pub vtable: usize,
}

// Just following the naming convention set forth by microsoft when it comes to ffi (see windows-rs)
#[allow(non_snake_case)]
impl EngineClient {
    pub unsafe fn GetScreenSize(&self) -> (i32, i32) {
        let get_screen_size = std::mem::transmute::<_, GetScreenSizeFn>(
            (self.vtable as *mut usize).offset(5).read() as *mut usize,
        );

        let mut screen_width = 0;
        let mut screen_height = 0;
        get_screen_size(
            self as *const Self,
            &mut screen_width as *mut i32,
            &mut screen_height as *mut i32,
        );

        (screen_width, screen_height)
    }

    // will show up in CDemoRecorder::WriteMessages
    // 000beacc  8b7590             mov     esi, dword [ebp-0x70 {var_74}]
    // 000beacf  8b9ed6fa3d00       mov     ebx, dword [esi+0x3dfad6]  {data_49e4a4}  {_g_pClientSidePrediction}
    // 000bead5  8b03               mov     eax, dword [ebx]  {_g_pClientSidePrediction}
    // 000bead7  8b08               mov     ecx, dword [eax]
    // 000bead9  8d55ac             lea     edx, [ebp-0x54 {var_5c+0x4}]  {"__text"}
    // 000beadc  89542404           mov     dword [esp+0x4 {var_88_2}], edx {var_5c+0x4}
    // 000beae0  890424             mov     dword [esp {var_8c_3}], eax
    // 000beae3  ff5124             call    dword [ecx+0x24]
    // 000beae6  8bbe46f73d00       mov     edi, dword [esi+0x3df746]  {data_49e114}
    // 000beaec  8b877c4b0000       mov     eax, dword [edi+0x4b7c]  {data_683e40} <----
    // 000beaf2  8945b8             mov     dword [ebp-0x48 {var_4c}], eax
    // 000beaf5  8b87804b0000       mov     eax, dword [edi+0x4b80]  {data_683e44} <----
    // 000beafb  8945bc             mov     dword [ebp-0x44 {var_4c+0x4}], eax
    // 000beafe  8b87844b0000       mov     eax, dword [edi+0x4b84]  {data_683e48} <----
    // 000beb04  8945c0             mov     dword [ebp-0x40 {var_4c+0x8}], eax
    // 000beb07  8b03               mov     eax, dword [ebx]  {_g_pClientSidePrediction}
    // 000beb09  8b08               mov     ecx, dword [eax]

    // look for
    // "Writing demo message %i bytes at..."

    pub unsafe fn GetViewAngles(&self) -> Vec3 {
        let get_view_angles = std::mem::transmute::<_, GetViewAnglesFn>(
            (self.vtable as *mut usize).offset(5).read() as *mut usize,
        );

        let mut view_angles: Vec3 = Default::default();

        get_view_angles(self as *const Self, &mut view_angles as *mut Vec3);

        view_angles
    }

    pub unsafe fn GetPlayerInfo(&self, player_info: *mut PlayerInfo) {
        let get_player_info = std::mem::transmute::<_, GetPlayerInfoFn>(
            (self.vtable as *mut usize).offset(8).read() as *mut usize,
        );

        get_player_info(self as *const Self, player_info);
    }

    pub unsafe fn IsInGame(&self) -> bool {
        let is_ingame = std::mem::transmute::<_, IsInGameFn>(
            (self.vtable as *mut usize).offset(26).read() as *mut usize,
        );

        is_ingame(self as *const Self)
    }

    pub unsafe fn GetMaxClients(&self) -> i32 {
        let is_ingame = std::mem::transmute::<_, GetMaxClientsFn>(
            (self.vtable as *mut usize).offset(21).read() as *mut usize,
        );

        is_ingame(self as *const Self)
    }

    pub unsafe fn ExecuteClientCmd(&self, command: *const c_char) {
        let execute_client_cmd = std::mem::transmute::<_, ExecuteClientCmdFn>(
            (self.vtable as *mut usize).offset(108).read() as *mut usize,
        );

        execute_client_cmd(self as *const Self, command)
    }
}

unsafe impl Send for EngineClient {}
