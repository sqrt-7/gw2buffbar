mod buffs;
mod compat;

use buffs::{BuffHandler, BuffIcon, SingleBuffConfig};
use compat::{DLL_FUNC, DLL_LIB};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static BUFF_HANDLER: Lazy<Mutex<BuffHandler>> = Lazy::new(|| Mutex::new(BuffHandler::new()));

arcdps::export!(
    name: "gw2buffbar",
    sig: 0x52247299,
    init,
    release,
    options_windows,
    options_end,
    imgui,
);

fn init() -> Result<(), String> {
    log::info!(target: "file", "gw2buffbar init");

    if DLL_LIB.is_none() || DLL_FUNC.is_none() {
        return Err("couldn't grab DLL function".to_string());
    }

    // todo
    let regen = SingleBuffConfig::new(
        718,
        200.0,
        200.0,
        BuffIcon::CicleOutline {
            radius: 20.0,
            thickness: 10.0,
            color: arc_util::colors::GREEN,
        },
    );

    let aegis = SingleBuffConfig::new(
        743,
        500.0,
        200.0,
        BuffIcon::CicleOutline {
            radius: 20.0,
            thickness: 10.0,
            color: [154.0 / 255.0, 54.0 / 255.0, 255.0 / 255.0, 1.0],
        },
    );

    let mut handler = BUFF_HANDLER.lock().unwrap();
    handler.add_buff(regen);
    handler.add_buff(aegis);

    Ok(())
}

fn release() {
    log::info!(target: "file", "gw2buffbar: stopped");
}

fn options_windows(ui: &arc_util::ui::Ui, window_name: Option<&str>) -> bool {
    if window_name.is_none() {
        BUFF_HANDLER.lock().unwrap().render_arcdps_options_main(ui);
    }

    false
}

fn options_end(ui: &arcdps::imgui::Ui) {
    BUFF_HANDLER.lock().unwrap().render_arcdps_options_tab(ui);
}

fn imgui(imgui_ui: &arcdps::imgui::Ui, _not_loading_or_character_selection: bool) {
    let mut handler = BUFF_HANDLER.lock().unwrap();
    if handler.is_visible() {
        handler.update_current_buffs();
        handler.render_buffs(imgui_ui);
    }
}
