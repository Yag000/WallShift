#[derive(serde::Deserialize)]
pub struct Settings {
    pub wallpaper_dir: String,
    pub betterlockscreen: bool,
    pub sleep_time: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            wallpaper_dir: format!(
                "{}/Pictures/Wallpapers",
                home::home_dir().unwrap().to_str().unwrap()
            ),
            betterlockscreen: false,
            sleep_time: 1800,
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // TODO: Add more possible sources
    // in particular $HOME/.config/wallpaper-updaterrc.yml
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "wallpaper-updaterrc.yml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}
