use windows::Win32::System::Threading::Sleep;

use crate::math::Vec3;

// Temp location
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClientClass {
    _pad_0: [u8; 0x14], //TODO: reverse more of this structure, addr(id) - 0x4 is another client class
    pub id: i32,
}

pub type GetClientClassFn = unsafe extern "thiscall" fn(*const BaseEntity) -> *mut ClientClass;
pub type GetAbsOriginFn = unsafe extern "thiscall" fn(*const BaseEntity) -> *mut Vec3;
pub type InLocalTeamFn = unsafe extern "thiscall" fn(*const BaseEntity) -> bool;
pub type GetRenderBoundsFn = unsafe extern "thiscall" fn(*const BaseEntity, *mut Vec3, *mut Vec3);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BaseEntity {
    pub vtable: usize,
    pub renderable: usize,
    pub networkable: usize,
}

impl BaseEntity {
    // From _C_BasePlayer_CreateObject
    //     7 @ 0049f644  eax_2 = arg2
    //     8 @ 0049f647  ecx_1 = arg1
    //     9 @ 0049f64a  edx_1 = esi->field_0xa4 <--- setup object func
    //    10 @ 0049f64c  var_14_1 = eax_2
    //    11 @ 0049f650  var_18_1 = ecx_1
    //    12 @ 0049f654  var_1c_1 = esi
    //    13 @ 0049f657  [edx_1].d(var_1c_1, var_18_1, var_14_1)  {"b"} <--- setup object
    //    14 @ 0049f65d  esi_1 = esi + 8
    //    15 @ 0049f660  eax_1 = esi_1
    //    16 @ 0049f660  goto 6 @ 0x49f668 <--- this is player class

    // 246 is a player, deduced by just printing out the only two entities on a 1 player local server lmao

    pub unsafe fn GetClientClass(&self) -> *mut ClientClass {
        let get_client_class = std::mem::transmute::<_, GetClientClassFn>(
            (self.networkable as *mut usize).offset(2).read() as *mut usize,
        );

        get_client_class(self as *const Self)
    }

    // C_BaseEntity::GetAbsOrigin calls into C_BaseEntity::CalcAbsolutePosition, see below listing.
    //

    pub unsafe fn GetAbsOrigin(&self) -> *mut Vec3 {
        let get_abs_origin = std::mem::transmute::<_, GetAbsOriginFn>(
            (self.vtable as *mut usize).offset(9).read() as *mut usize,
        );

        get_abs_origin(self as *const Self)
    }

    pub unsafe fn InLocalTeam(&self) -> bool {
        let in_local_team = std::mem::transmute::<_, InLocalTeamFn>(
            (self.vtable as *mut usize).offset(78).read() as *mut usize,
        );

        in_local_team(self as *const Self)
    }

    pub unsafe fn IsAlive(&self) -> bool {
        let is_alive = std::mem::transmute::<_, InLocalTeamFn>(
            (self.vtable as *mut usize).offset(78).read() as *mut usize,
        );

        is_alive(self as *const Self)
    }

    // Basically, C_BaseObject::GetTargetIDString -> C_BasePlayer::GetPlayerName
    // pub unsafe fn GetPlayerName(&self) -> String {
    //     let get_player_name = std::mem::transmute::<_, GetPlayerNameFn>(
    //         (self.vtable as *mut usize).offset(0).read() as *mut usize,
    //     );

    //     get_player_name(self as *const Self)
    // }

    pub unsafe fn GetRenderBounds(&self, min: *mut Vec3, max: *mut Vec3) {
        let get_render_bounds = std::mem::transmute::<_, GetRenderBoundsFn>(
            (self.renderable as *mut usize).offset(20).read() as *mut usize,
        );

        get_render_bounds(self as *const Self, min, max);
    }
}
