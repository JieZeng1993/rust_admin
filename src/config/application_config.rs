use getset::{Getters, Setters};
use once_cell::sync::Lazy;

use crate::config::domain::*;

/// Config
#[derive(Getters, Setters, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfig {
    #[getset(get = "pub", set)]
    app: AppConfig,
    #[getset(get = "pub", set = "pub")]
    login: LoginConfig,
    #[getset(get = "pub", set = "pub")]
    database: DatabaseConfig,
    #[getset(get = "pub", set = "pub")]
    log: LogConfig,
    #[getset(get = "pub", set = "pub")]
    cache: CacheConfig,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../application.yml");
        //load config
        let result: ApplicationConfig =
            serde_yaml::from_str(yml_data).expect("load config file fail");

        let app_config = result.app();
        println!("[{}] load config:{:#?}", app_config.name(), result);

        if *result.app().debug() {
            println!("[{}] ///////////////////// Start On Debug Mode ////////////////////////////", app_config.name());
        } else {
            println!("[{}] ///////////////////// Start On Release Mode ////////////////////////////", app_config.name());
        }
        result
    }
}

pub static APPLICATION_CONFIG: Lazy<ApplicationConfig> = Lazy::new(|| ApplicationConfig::default());