#![allow(dead_code)]

use crate::{
    compat::{get_len, DLL_FUNC, DLL_LIB},
    config::LocalConfigItem,
};
use arc_util::ui::Ui;
use arcdps::imgui::Condition;
use std::{collections::HashMap, time::Instant};

const TICK: u128 = 100;

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
    TriangleUp {
        side_length: f32,
        color: RGBColor,
    },
}

#[derive(Clone, Debug)]
pub struct SingleBuffConfig {
    buff_id: u32,
    window_x: f32,
    window_y: f32,
    opt_title: String,
    icon: BuffIcon,
    active_stacks: i32,
    show_stacks: bool,
}

impl SingleBuffConfig {
    pub fn new(
        buff_id: u32,
        window_x: f32,
        window_y: f32,
        opt_title: String,
        icon: BuffIcon,
        show_stacks: bool,
    ) -> Self {
        SingleBuffConfig {
            buff_id,
            window_x,
            window_y,
            opt_title,
            icon,
            active_stacks: 0i32,
            show_stacks,
        }
    }

    pub fn draw(&self, ui: &Ui) {
        match self.icon {
            BuffIcon::CircleOutline {
                radius,
                thickness,
                color,
            } => self.draw_circle_outline(ui, radius, thickness, color),
            BuffIcon::TriangleUp { side_length, color } => {
                self.draw_triangle_up(ui, side_length, color)
            }
        }
    }

    fn new_window(
        &self,
        title: String,
        show_title: bool,
        pos: [f32; 2],
        size: [f32; 2],
    ) -> arcdps::imgui::Window<String> {
        arcdps::imgui::Window::new(title)
            .position(pos, Condition::Always)
            .size(size, Condition::Always)
            .resizable(false)
            .focus_on_appearing(false)
            .no_nav()
            .title_bar(show_title)
            .draw_background(true)
            .collapsible(false)
    }

    fn draw_circle_outline(&self, ui: &Ui, radius: f32, thickness: f32, color: RGBColor) {
        let px = self.window_x + radius + thickness;
        let mut py = self.window_y + radius + thickness;
        let window_length = radius * 2.0 + thickness * 2.0;
        let mut window_size = [window_length, window_length];

        // Modify dimensions to fit the header
        let mut title = format!("{}", self.buff_id);
        let mut show_title = false;
        if self.opt_title != "" {
            title = self.opt_title.clone();
            show_title = true;
            window_size[1] += 20.0;
            py += 20.0;
        }

        let win = self.new_window(
            title,
            show_title,
            [self.window_x, self.window_y],
            window_size,
        );

        win.build(&ui, || {
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_circle([px, py], radius, color.to_imgui_color())
                .thickness(thickness)
                .build();

            if self.show_stacks {
                let text_size = ui.calc_text_size(self.active_stacks.to_string());
                let text_pos = [px - text_size[0] / 2.0, py - text_size[1] / 2.0];
                draw_list.add_text(
                    text_pos,
                    [1.0, 1.0, 1.0, 1.0],
                    self.active_stacks.to_string(),
                )
            }
        });
    }

    fn draw_triangle_up(&self, ui: &Ui, side_length: f32, color: RGBColor) {
        /*
               B

           A       C
        */

        let thickness = 1.0;
        let window_length = side_length + thickness * 2.0;

        let mut tri_a = [
            self.window_x + thickness + 2.0,
            self.window_y + (window_length - thickness - 2.0),
        ];
        let mut tri_c = [
            self.window_x + (window_length - thickness - 2.0),
            self.window_y + (window_length - thickness - 2.0),
        ];
        let mut tri_b = [
            tri_a[0] + ((tri_c[0] - tri_a[0]) / 2.0),
            self.window_y + thickness + 2.0,
        ];

        let mut window_size = [window_length, window_length];

        // Modify dimensions to fit the header
        let mut title = format!("{}", self.buff_id);
        let mut show_title = false;
        if !self.opt_title.is_empty() {
            title = self.opt_title.clone();
            show_title = true;
            window_size[1] += 22.0;
            tri_a[1] += 22.0;
            tri_b[1] += 22.0;
            tri_c[1] += 22.0;
        }

        let win = self.new_window(
            title,
            show_title,
            [self.window_x, self.window_y],
            window_size,
        );

        win.build(&ui, || {
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_triangle(tri_a, tri_b, tri_c, color.to_imgui_color())
                .filled(true)
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
                crate::config::LocalConfigItemIcon::TriangleUp { side_length, color } => {
                    BuffIcon::TriangleUp {
                        side_length: side_length as f32,
                        color: color.try_into()?,
                    }
                }
            }
        };

        Ok(SingleBuffConfig::new(
            value.buff_id,
            value.window_pos[0],
            value.window_pos[1],
            value.title.clone(),
            icon,
            value.show_stacks,
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

    pub fn show_mouse(&self, ui: &arcdps::imgui::Ui) {
        if ui.io().mouse_down[0] == true
            || ui.io().mouse_down[1] == true
            || ui.io().mouse_down[2] == true
        {
            ui.get_foreground_draw_list()
                .add_circle(ui.io().mouse_pos, 30.0, [1.0, 0.0, 0.4, 1.0])
                .thickness(10.0)
                .filled(false)
                .build();
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
                    let mut x = (*conf).clone();
                    x.active_stacks = buff.count;
                    self.last_output.push(x);
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
