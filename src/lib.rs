mod buffs;
mod compat;
mod config;

use buffs::{BuffHandler, SingleBuffConfig};
use compat::{DLL_FUNC, DLL_LIB};
use config::LocalConfig;
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
    log::info!(target: "file", "init");

    if DLL_LIB.is_none() || DLL_FUNC.is_none() {
        return Err("couldn't grab DLL function".to_string());
    }

    let mut handler = BUFF_HANDLER.lock().unwrap();

    match LocalConfig::new_from_file("addons/arcdps/gw2buffbar.json") {
        Err(e) => return Err(format!("failed to load gw2buffbar.json (error: {})", e)),
        Ok(conf) => {
            for item in conf.items.iter() {
                let conv: SingleBuffConfig = item.try_into()?;
                log::info!(target: "file", "config item: {:?}", conv);
                handler.watch_buff(conv);
            }
        }
    }

    Ok(())
}

fn release() {
    log::info!(target: "file", "stopped");
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
    handler.update_current_buffs(imgui_ui);
}
