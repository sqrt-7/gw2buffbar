#![allow(dead_code)]

use crate::{
    compat::{get_len, DLL_FUNC, DLL_LIB},
    config::LocalConfigItem,
};
use arc_util::ui::Ui;
use arcdps::imgui::Condition;
use std::{collections::HashMap, time::Instant};

const TICK: u128 = 100;

// #[derive(Clone, Copy, Debug, FromPrimitive)]
// pub enum BuffID {
//     Protection = 717,
//     Regeneration = 718,
//     Swiftness = 719,
//     Fury = 725,
//     Vigor = 726,
//     Might = 740,
//     Aegis = 743,
//     Retaliation = 873,
//     Stability = 1122,
//     Quickness = 1187,
//     Resistance = 26980,
// }

// // impl ToString for BuffID {
// //     fn to_string(&self) -> String {
// //         format!("{:?}", self)
// //     }
// // }

// // impl Into<u32> for BuffID {
// //     fn into(self) -> u32 {
// //         self as u32
// //     }
// // }

// impl TryFrom<u32> for BuffID {
//     type Error = String;

//     fn try_from(value: u32) -> Result<Self, Self::Error> {
//         match num::FromPrimitive::from_u32(value) {
//             Some(v) => Ok(v),
//             None => Err(format!("unknown buff_id: {}", value)),
//         }
//     }
// }

#[derive(Clone, Copy, Debug)]
pub struct RGBColor {
    r: u32,
    g: u32,
    b: u32,
}

impl RGBColor {
    fn to_imgui_color(&self) -> [f32; 4] {
        return [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            1.0,
        ];
    }
}

impl ToString for RGBColor {
    fn to_string(&self) -> String {
        format!("r:{},g:{},b:{}", self.r, self.g, self.b)
    }
}

impl TryFrom<[u32; 3]> for RGBColor {
    type Error = String;

    fn try_from(value: [u32; 3]) -> Result<Self, Self::Error> {
        if value[0] > 255 || value[1] > 255 || value[2] > 255 {
            return Err("color values must be between 0-255".to_string());
        }

        Ok(RGBColor {
            r: value[0],
            g: value[1],
            b: value[2],
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BuffIcon {
    CircleOutline {
        radius: f32,
        thickness: f32,
        color: RGBColor,
    },
}

#[derive(Clone, Debug)]
pub struct SingleBuffConfig {
    buff_id: u32,
    pos_x: f32,
    pos_y: f32,
    opt_title: String,
    icon: BuffIcon,
}

impl SingleBuffConfig {
    pub fn new(buff_id: u32, pos_x: f32, pos_y: f32, opt_title: String, icon: BuffIcon) -> Self {
        SingleBuffConfig {
            buff_id,
            pos_x,
            pos_y,
            opt_title,
            icon,
        }
    }

    pub fn draw(&self, ui: &Ui) {
        match self.icon {
            BuffIcon::CircleOutline {
                radius,
                thickness,
                color,
            } => self.draw_circle_outline(ui, radius, thickness, color),
        }
    }

    fn draw_circle_outline(&self, ui: &Ui, radius: f32, thickness: f32, color: RGBColor) {
        let px = self.pos_x + radius + thickness;
        let mut py = self.pos_y + radius + thickness;
        let win_width = radius * 2.0 + thickness * 2.0;
        let mut win_height = radius * 2.0 + thickness * 2.0;

        let mut title = format!("{}", self.buff_id);
        let mut show_title = false;

        if self.opt_title != "" {
            title = self.opt_title.clone();
            show_title = true;
            win_height += 20.0;
            py += 20.0;
        }

        let win = arcdps::imgui::Window::new(title)
            .position([self.pos_x, self.pos_y], Condition::Always)
            .size([win_width, win_height], Condition::Always)
            .resizable(false)
            .focus_on_appearing(false)
            .no_nav()
            .title_bar(show_title)
            .draw_background(false)
            .collapsible(false);

        win.build(&ui, || {
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_circle([px, py], radius, color.to_imgui_color())
                .thickness(thickness)
                .build();
        });
    }
}

impl TryFrom<&LocalConfigItem> for SingleBuffConfig {
    type Error = String;

    fn try_from(value: &LocalConfigItem) -> Result<Self, Self::Error> {
        let icon = {
            match value.icon {
                crate::config::LocalConfigItemIcon::CircleOutline {
                    radius,
                    thickness,
                    color,
                } => BuffIcon::CircleOutline {
                    radius: radius as f32,
                    thickness: thickness as f32,
                    color: color.try_into()?,
                },
            }
        };

        Ok(SingleBuffConfig::new(
            value.buff_id,
            value.window_pos[0],
            value.window_pos[1],
            value.title.clone(),
            icon,
        ))
    }
}

pub struct BuffHandler {
    is_visible: bool,
    last_process: Instant,
    last_output: Vec<SingleBuffConfig>,
    registry: HashMap<u32, SingleBuffConfig>,
}

impl BuffHandler {
    pub fn new() -> Self {
        BuffHandler {
            is_visible: false,
            last_process: Instant::now(),
            last_output: Vec::new(),
            registry: HashMap::new(),
        }
    }

    pub fn watch_buff(&mut self, config: SingleBuffConfig) {
        self.registry.insert(config.buff_id.into(), config);
    }

    pub fn update_current_buffs(&mut self, ui: &arcdps::imgui::Ui) {
        if self.is_visible {
            if DLL_LIB.is_none() || DLL_FUNC.is_none() {
                return self.render_buffs(ui);
            }

            // Only run after {TICK} ms time has passed since last
            let elapsed = self.last_process.elapsed().as_millis();
            if elapsed < TICK {
                return self.render_buffs(ui);
            }

            self.last_process = Instant::now();

            let func = DLL_FUNC.as_ref().unwrap();
            let buffs = func();
            let len = unsafe { get_len(buffs) };

            let slice = unsafe { std::slice::from_raw_parts(buffs, len) };

            self.last_output.clear();
            for buff in slice.iter() {
                if let Some(conf) = self.registry.get(&(buff.id)) {
                    self.last_output.push((*conf).clone());
                }
            }

            self.render_buffs(ui);
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

    fn render_buffs(&self, ui: &arcdps::imgui::Ui) {
        for buff in self.last_output.iter() {
            buff.draw(ui);
        }
    }
}
