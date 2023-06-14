use crate::interfaces::panel::Panel;

pub mod panel;

pub fn install_hooks() {
    log::info!("installing hooks.");
    unsafe {
        log::info!(
            "paint traverse fn: {:?}",
            (crate::I_PANEL.get().unwrap().vtable as *mut usize)
                .offset(41)
                .read() as *mut usize
        );
        let paint_traverse_target = std::mem::transmute::<_, panel::FnPaintTraverse>(
            (crate::I_PANEL.get().unwrap().vtable as *mut usize)
                .offset(41)
                .read() as *mut usize,
        );
        panel::PaintTraverseHook
            .initialize(paint_traverse_target, panel::paint_traverse_detour)
            .unwrap()
            .enable()
            .unwrap();
    }
}

pub fn remove_hooks() {
    log::info!("removing hooks.");
    unsafe {
        panel::PaintTraverseHook.disable().unwrap();
    }
}
