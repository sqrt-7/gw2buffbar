use arcdps::{exports, imgui};

pub struct WindowHandler {
    pub is_main_visible: bool,
}

impl WindowHandler {
    pub fn new() -> Self {
        WindowHandler {
            is_main_visible: false,
        }
    }

    pub fn render_arcdps_options_main(&mut self, ui: &imgui::Ui) {
        ui.checkbox("GW2BuffBar", &mut self.is_main_visible);
    }

    pub fn render_arcdps_options_tab(&self, ui: &imgui::Ui) {
        let colors = exports::colors();
        let grey = colors
            .core(exports::CoreColor::MediumGrey)
            .unwrap_or(arc_util::colors::GREY);
        // let red = colors
        //     .core(exports::CoreColor::LightRed)
        //     .unwrap_or(arc_util::colors::RED);
        // let green = colors
        //     .core(exports::CoreColor::LightGreen)
        //     .unwrap_or(arc_util::colors::GREEN);
        // let yellow = colors
        //     .core(exports::CoreColor::LightYellow)
        //     .unwrap_or(arc_util::colors::YELLOW);

        //const SPACING: f32 = 5.0;
        //let _style = arc_util::ui::render::small_padding(ui);
        //let _input_width = arc_util::ui::render::ch_width(ui, 16);

        ui.spacing();
        ui.text_colored(grey, "blah blah");
        ui.checkbox("I'm a checkbox", &mut true);

        if ui.button("Test") {
            log::info!(target: "both", "BUTTON_PRESS")
        }
    }

    pub fn render_main_window(&mut self, ui: &imgui::Ui, data: Vec<String>) {
        if self.is_main_visible {
            imgui::Window::new("GW2 BuffBar")
                .always_auto_resize(true)
                .focus_on_appearing(false)
                .no_nav()
                .title_bar(true)
                .draw_background(true)
                .collapsible(false)
                .opened(&mut self.is_main_visible)
                .build(ui, || {
                    imgui::TabBar::new("main_tabs").build(ui, || {
                        for v in data.iter() {
                            ui.text(v);
                        }
                    });
                });
        }
    }
}
