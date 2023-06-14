#![feature(abi_thiscall)]
#![feature(new_uninit)]
#![feature(maybe_uninit_extra)]

use std::{
    ffi::{c_void, CStr},
    fs::OpenOptions,
    os::windows::prelude::AsRawHandle,
    slice::SliceIndex,
    time::Duration,
};

use once_cell::sync::OnceCell;

// TODO: switch flexi_logger out for a non-blocking tracing logger
use flexi_logger::{FileSpec, Logger};
use interfaces::{
    debug_overlay::DebugOverlay, engine_client::EngineClient, entity_list::EntityList,
    panel::Panel, surface::Surface,
};
use windows::Win32::{
    Foundation::{BOOL, HANDLE, HINSTANCE, PSTR, PWSTR},
    System::{
        Console::{AllocConsole, SetStdHandle, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE},
        LibraryLoader::{
            DisableThreadLibraryCalls, FreeLibraryAndExitThread, GetModuleHandleA,
            GetModuleHandleW, GetProcAddress,
        },
        Threading::CreateThread,
    },
    UI::Input::KeyboardAndMouse::GetAsyncKeyState,
};

use crate::{interfaces::get_interface, math::Vec3};

pub static I_ENGINECLIENT: OnceCell<&'static EngineClient> = OnceCell::new();
pub static I_PANEL: OnceCell<&'static Panel> = OnceCell::new();
pub static I_ENTITYLIST: OnceCell<&'static EntityList> = OnceCell::new();
pub static I_SURFACE: OnceCell<&'static Surface> = OnceCell::new();
pub static I_DEBUGOVERLAY: OnceCell<&'static DebugOverlay> = OnceCell::new();

pub mod entity;
pub mod hooks;
pub mod interfaces;
pub mod macros;
pub mod math;
pub mod mem;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(module: HINSTANCE, reason: u32, _reserved: *const u8) -> BOOL {
    match reason {
        1 => {
            // DLL_PROCESS_ATTACH
            DisableThreadLibraryCalls(module);

            let mut locheck = 0;
            while GetModuleHandleA(PSTR(win_str_ptr!("serverbrowser.dll"))) == 0 {
                if locheck > 10 {
                    FreeLibraryAndExitThread(module, 1);
                }
                std::thread::sleep(Duration::from_millis(3333));
                locheck += 1;
            }

            // TODO: switch over to `CreateSimpleThread` to call engine functions directly
            // GetProcAddress(std::ptr::null_mut(), "CreateSimpleThread");

            if AllocConsole().as_bool() {
                let file = OpenOptions::new()
                    .write(true)
                    .read(true)
                    .open("CONOUT$")
                    .unwrap();

                SetStdHandle(STD_OUTPUT_HANDLE, HANDLE(file.as_raw_handle() as _));
                SetStdHandle(STD_ERROR_HANDLE, HANDLE(file.as_raw_handle() as _));
                std::mem::forget(file);
            }

            // spotted inconsistency in the start address, follow up
            CreateThread(
                std::ptr::null_mut() as *const _,
                0,
                Some(main_thread),
                module as *const _,
                0,
                std::ptr::null_mut(),
            );

            return BOOL(1);
        }
        0 => {
            // DLL_PROCESS_DETACH
            // unload hooks
            detach();
        }
        _ => {}
    }
    BOOL(0)
}

unsafe extern "system" fn main_thread(module: *mut c_void) -> u32 {
    Logger::try_with_str("debug")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("rustinternal") // create files in folder ./rustinternal (relative to hl2.exe)
                .basename("internal")
                .suffix("log"),
        )
        .duplicate_to_stderr(flexi_logger::Duplicate::Debug)
        .print_message()
        .start()
        .unwrap();

    log::info!("main thread started.");

    setup_interfaces();

    log::info!(
        "screensize: {:?}",
        I_ENGINECLIENT.get().unwrap().GetScreenSize()
    );

    log::info!("ingame: {:?}", I_ENGINECLIENT.get().unwrap().IsInGame());

    log::info!("to initialize hooks press INSERT");

    // init hooks when hit insert key
    while GetAsyncKeyState(0x2D) == 0_i16 {}

    hooks::install_hooks();

    // exit when hit delete key
    while GetAsyncKeyState(0x2E) == 0_i16 {}

    // after this we have a panic, this can be disabled in prod through a panic hook
    exit_dis_hoe();
    FreeLibraryAndExitThread(module as HINSTANCE, 0); // dwexitcode SUCCESS
    1
}

pub fn setup_interfaces() {
    // all of these are unsafe so writing wrappers in the future (which would also serve as the exposed functions to duktape, as they would be safe to call) to handle all edge cases with correct return value encapsulation would greatly improve user experience.

    log::info!("setting up interfaces.");

    I_ENGINECLIENT
        .set(get_interface::<EngineClient>(
            get_module("engine.dll"),
            "VEngineClient013",
        ))
        .unwrap();

    I_PANEL
        .set(get_interface::<Panel>(
            get_module("vgui2.dll"),
            "VGUI_Panel009",
        ))
        .unwrap();

    I_ENTITYLIST
        .set(get_interface::<EntityList>(
            get_module("client.dll"),
            "VClientEntityList003",
        ))
        .unwrap();

    I_SURFACE
        .set(get_interface::<Surface>(
            get_module("vguimatsurface.dll"),
            "VGUI_Surface030",
        ))
        .unwrap();

    I_DEBUGOVERLAY
        .set(get_interface::<DebugOverlay>(
            get_module("engine.dll"),
            "VDebugOverlay003",
        ))
        .unwrap();
}

pub fn get_module(module_name: &str) -> HINSTANCE {
    unsafe {
        let module: HINSTANCE = GetModuleHandleA(PSTR(win_str_ptr!("{}", module_name)));
        return module;
    }
}

pub fn detach() {
    hooks::remove_hooks();
}
