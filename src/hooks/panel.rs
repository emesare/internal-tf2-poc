use detour::static_detour;
use std::ffi::CStr;
use std::os::raw::c_char;
use windows::Win32::System::Threading::Sleep;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

use crate::{
    entity::BaseEntity, math::Vec3, I_DEBUGOVERLAY, I_ENGINECLIENT, I_ENTITYLIST, I_PANEL,
    I_SURFACE,
};

static_detour! {
    pub static PaintTraverseHook: unsafe extern "thiscall" fn(
        *mut usize,
        u32,
        bool,
        bool
    );
}

pub type FnPaintTraverse = unsafe extern "thiscall" fn(
    this: *mut usize,
    panel: u32,
    force_repaint: bool,
    allow_force: bool,
);

pub fn paint_traverse_detour(
    this: *mut usize,
    panel_id: u32,
    force_repaint: bool,
    allow_force: bool,
) {
    unsafe {
        // Will be used for drawing
        static mut PANEL_ID: u32 = 0;
        // Will be implemented later for no scope
        static mut PANEL_HUD_ID: u32 = 0;

        let interface = crate::I_PANEL.get().unwrap();
        //interface.GetName(this);

        // VPanelWrapper::GetName : 559988e0
        // Interesting: XPBar, CyclingAd (AdContainer)
        // MatSystemTop is ESC drawing under the tab menu of tf2

        // let panel_name = CStr::from_ptr(interface.GetName(panel_id))
        //     .to_str()
        //     .unwrap();

        //log::debug!("{}", panel_name);

        // The current panel is what i prefer, only shows esp while playing, you can switch this to whatever but I think this is the best
        if CStr::from_ptr(interface.GetName(panel_id))
            .to_str()
            .unwrap()
            .contains("FocusOverlayPanel")
        {
            let i = crate::I_SURFACE.get().unwrap();
            i.DrawSetColor(255, 0, 255, 255);
            i.DrawFilledRect(50, 50, 80, 80);
            // havent registered a font to use yet!
            i.DrawSetTextColor(255, 0, 255, 255);
            i.DrawText("Writteninrust - a professionally coded hack", 300, 300);

            // TODO: make an iterator for this so we can while entities.next is much clean codenz
            let engine_client = I_ENGINECLIENT.get().unwrap();
            if engine_client.IsInGame() {
                let entity_list = I_ENTITYLIST.get().unwrap();

                for entity_num in 0..engine_client.GetMaxClients() {
                    if let Some(entity) = entity_list.GetClientEntity(entity_num as usize).as_ref()
                    {
                        if entity.GetClientClass().as_ref().unwrap().id == 246 {
                            if entity.InLocalTeam() {
                                // Teammate!!!!
                                i.DrawSetColor(0, 54, 230, 255);
                            } else {
                                // Enemy!!!!
                                i.DrawSetColor(78, 39, 64, 255);
                            }

                            let debug_overlay = I_DEBUGOVERLAY.get().unwrap();
                            let mut origin_screen = Vec3::default(); // this is really a Vec2
                            if debug_overlay.ScreenPosition(
                                entity.GetAbsOrigin(),
                                &mut origin_screen as *mut Vec3,
                            ) == 0
                            {
                                // latest (and greatest) esp technology
                                i.DrawFilledRect(
                                    origin_screen.x() - 3,
                                    origin_screen.y() - 3,
                                    origin_screen.x() + 3,
                                    origin_screen.y() + 3,
                                );
                            }
                        }
                    }
                }
            }
        }

        PaintTraverseHook.call(this, panel_id, force_repaint, allow_force);
    }
}
