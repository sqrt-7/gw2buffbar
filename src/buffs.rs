use crate::compat::{get_len, DLL_FUNC, DLL_LIB};
use arc_util::ui::Ui;
use arcdps::imgui::Condition;
use once_cell::sync::Lazy;
use std::{collections::HashMap, time::Instant};

const TICK: u128 = 100;

static BUFF_MAP: Lazy<HashMap<u32, String>> = Lazy::new(|| {
    HashMap::from([
        (717, "Protection".to_owned()),
        (718, "Regeneration".to_owned()),
        (719, "Swiftness".to_owned()),
        (725, "Fury".to_owned()),
        (726, "Vigor".to_owned()),
        (740, "Might".to_owned()),
        (743, "Aegis".to_owned()),
        (873, "Retaliation".to_owned()),
        (1122, "Stability".to_owned()),
        (1187, "Quickness".to_owned()),
        (26980, "Resistance".to_owned()),
    ])
});

#[derive(Clone)]
pub enum BuffIcon {
    CicleOutline {
        radius: f32,
        thickness: f32,
        color: [f32; 4],
    },
}

#[derive(Clone)]
pub struct SingleBuffConfig {
    buff_id: u32,
    pos_x: f32,
    pos_y: f32,
    icon: BuffIcon,
}

impl SingleBuffConfig {
    pub fn new(buff_id: u32, pos_x: f32, pos_y: f32, icon: BuffIcon) -> Self {
        SingleBuffConfig {
            buff_id,
            pos_x,
            pos_y,
            icon,
        }
    }

    pub fn draw(&self, ui: &Ui) {
        match self.icon {
            BuffIcon::CicleOutline {
                radius,
                thickness,
                color,
            } => self.draw_circle_outline(ui, radius, thickness, color),
        }
    }

    fn draw_circle_outline(&self, ui: &Ui, radius: f32, thickness: f32, color: [f32; 4]) {
        let px = self.pos_x + radius + thickness;
        let py = self.pos_y + radius + thickness;
        let win_width = radius * 2.0 + thickness * 2.0;
        let win_height = radius * 2.0 + thickness * 2.0;

        let win = arcdps::imgui::Window::new(self.buff_id.to_string())
            .position([self.pos_x, self.pos_y], Condition::Always)
            .size([win_width, win_height], Condition::Always)
            .resizable(false)
            .focus_on_appearing(false)
            .no_nav()
            .title_bar(false)
            .draw_background(false)
            .collapsible(false);

        win.build(&ui, || {
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_circle([px, py], radius, color)
                .thickness(thickness)
                .build();
        });
    }
}

pub struct BuffHandler {
    is_visible: bool,
    last_process: Instant,
    last_output: Vec<SingleBuffConfig>,
    config: HashMap<u32, SingleBuffConfig>,
}

impl BuffHandler {
    pub fn new() -> Self {
        BuffHandler {
            is_visible: false,
            last_process: Instant::now(),
            last_output: Vec::new(),
            config: HashMap::new(),
        }
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn add_buff(&mut self, config: SingleBuffConfig) {
        self.config.insert(config.buff_id, config);
    }

    pub fn update_current_buffs(&mut self) {
        if DLL_LIB.is_none() || DLL_FUNC.is_none() {
            return;
        }

        let elapsed = self.last_process.elapsed().as_millis();
        if elapsed < TICK {
            return;
        }

        self.last_process = Instant::now();

        let func = DLL_FUNC.as_ref().unwrap();
        let buffs = func();
        let len = unsafe { get_len(buffs) };

        let slice = unsafe { std::slice::from_raw_parts(buffs, len) };

        self.last_output.clear();
        for buff in slice.iter() {
            if let Some(conf) = self.config.get(&(buff.id)) {
                self.last_output.push((*conf).clone());
            }
        }
    }

    pub fn render_arcdps_options_main(&mut self, ui: &arcdps::imgui::Ui) {
        ui.checkbox("GW2BuffBar", &mut self.is_visible);
    }

    pub fn render_arcdps_options_tab(&self, ui: &arcdps::imgui::Ui) {
        let colors = arcdps::exports::colors();
        let grey = colors
            .core(arcdps::exports::CoreColor::MediumGrey)
            .unwrap_or(arc_util::colors::GREY);
        ui.spacing();
        ui.text_colored(grey, "blah blah");
        ui.checkbox("I'm a checkbox", &mut true);

        if ui.button("Test") {
            log::info!(target: "both", "BUTTON_PRESS")
        }
    }

    pub fn render_buffs(&self, ui: &arcdps::imgui::Ui) {
        if self.is_visible {
            for buff in self.last_output.iter() {
                buff.draw(ui);
            }
        }
    }
}
