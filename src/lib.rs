mod buffs;
mod compat;
mod ui;

use arcdps::imgui;
use buffs::BuffHandler;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use ui::WindowHandler;

static BUFF_HANDLER: Lazy<Mutex<Option<BuffHandler>>> =
    Lazy::new(|| Mutex::new(BuffHandler::new()));
static WINDOW_HANDLER: Lazy<Mutex<WindowHandler>> = Lazy::new(|| Mutex::new(WindowHandler::new()));

arcdps::export!(
    name: "gw2buffbar",
    sig: 0x52247299,
    init,
    options_windows,
    options_end,
    imgui,
);

fn init() -> Result<(), String> {
    log::info!(target: "file", "gw2buffbar init");

    if BUFF_HANDLER.lock().unwrap().is_none() {
        log::info!(target: "file", "gw2buffbar -- BUFF_HANDLER failed");
    }

    Ok(())
}

fn options_windows(ui: &arc_util::ui::Ui, window_name: Option<&str>) -> bool {
    if window_name.is_none() {
        WINDOW_HANDLER
            .lock()
            .unwrap()
            .render_arcdps_options_main(ui);
    }

    false
}

fn options_end(ui: &arcdps::imgui::Ui) {
    WINDOW_HANDLER.lock().unwrap().render_arcdps_options_tab(ui);
}

fn imgui(imgui_ui: &imgui::Ui, _not_loading_or_character_selection: bool) {
    let mut data = BUFF_HANDLER.lock().unwrap();

    if data.is_some() {
        let data = data.as_mut().unwrap().getbuffs();

        if let Err(e) = data {
            log::info!(target: "file", "gw2buffbar.getbuffs() error: {}", e);
            return;
        }

        WINDOW_HANDLER
            .lock()
            .unwrap()
            .render_main_window(imgui_ui, data.unwrap());
    }
}
