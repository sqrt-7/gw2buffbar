#[derive(serde::Deserialize)]
pub struct LocalConfig {
    pub items: Vec<LocalConfigItem>,
}

#[derive(serde::Deserialize)]
pub struct LocalConfigItem {
    pub buff_id: u32,
    pub window_pos: [f32; 2],
    pub icon: LocalConfigItemIcon,

    // Optional fields
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub show_stacks: bool,
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
    #[serde(rename = "triangle_up")]
    TriangleUp { side_length: u32, color: [u32; 3] },
}

impl LocalConfig {
    pub fn new_from_file(filepath: &str) -> Result<Self, config::ConfigError> {
        let loader = config::Config::builder()
            .add_source(config::File::new(filepath, config::FileFormat::Json))
            .build()?;

        loader.try_deserialize::<LocalConfig>()
    }
}
