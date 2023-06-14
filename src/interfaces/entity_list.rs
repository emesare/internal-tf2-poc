use crate::entity::BaseEntity;

// TODO: change return pointers to Entity pointers

pub type GetClientEntityFn =
    unsafe extern "thiscall" fn(this: *const EntityList, entity_num: usize) -> *mut BaseEntity;
pub type GetClientEntityFromHandleFn =
    unsafe extern "thiscall" fn(this: *const EntityList, entity_handle: usize) -> *mut BaseEntity;

#[derive(Debug)]
pub struct EntityList {
    pub vtable: usize,
}

// Just following the naming convention set forth by microsoft when it comes to ffi (see windows-rs)
#[allow(non_snake_case)]
impl EntityList {
    pub unsafe fn GetClientEntity(&self, entity_num: usize) -> *mut BaseEntity {
        let get_client_entity = std::mem::transmute::<_, GetClientEntityFn>(
            (self.vtable as *mut usize).offset(3).read() as *mut usize,
        );

        get_client_entity(self as *const Self, entity_num)
    }

    pub unsafe fn GetClientEntityFromHandle(&self, entity_handle: usize) -> *mut BaseEntity {
        let get_client_entity_from_handle = std::mem::transmute::<_, GetClientEntityFromHandleFn>(
            (self.vtable as *mut usize).offset(4).read() as *mut usize,
        );

        get_client_entity_from_handle(self as *const Self, entity_handle)
    }
}

unsafe impl Send for EntityList {}
