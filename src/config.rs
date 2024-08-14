#[derive(serde::Deserialize)]
pub struct LocalConfig {
    pub items: Vec<LocalConfigItem>,
}

#[derive(serde::Deserialize)]
pub struct LocalConfigItem {
    pub buff_id: u32,
    pub window_pos: [f32; 2],
    pub title: String,
    pub icon: LocalConfigItemIcon,
}

#[derive(Clone, serde::Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum LocalConfigItemIcon {
    #[serde(rename = "circle_outline")]
    CircleOutline {
        radius: u32,
        thickness: u32,
        color: [u32; 3],
    },
}

impl LocalConfig {
    pub fn new_from_file(filepath: &str) -> Result<Self, config::ConfigError> {
        let loader = config::Config::builder()
            .add_source(config::File::new(filepath, config::FileFormat::Json))
            .build()?;

        loader.try_deserialize::<LocalConfig>()
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::buffs::SingleBuffConfig;

//     use super::LocalConfig;

//     #[test]
//     fn load_from_file() -> Result<(), String> {
//         match LocalConfig::new_from_file("gw2buffbar.json") {
//             Err(e) => Err(format!("failed to load gw2buffbar.json (error: {})", e)),
//             Ok(conf) => {
//                 for item in conf.items.iter() {
//                     let conv: SingleBuffConfig = item.try_into()?;
//                     println!("config item: {:?}", conv);
//                 }
//                 Ok(())
//             }
//         }
//     }
// }
