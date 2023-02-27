use getset::{Getters, Setters};

/// Config domain
#[derive(Getters, Setters, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    #[getset(get = "pub", set = "pub")]
    port: u16,
    #[getset(get = "pub", set = "pub")]
    name: String,
    #[getset(get = "pub", set = "pub")]
    uk: String,
    #[getset(get = "pub", set = "pub")]
    debug: bool,
}

#[derive(Getters, Setters, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginConfig {
    #[getset(get = "pub", set = "pub")]
    login_fail_retry:  String,
    #[getset(get = "pub", set = "pub")]
    login_fail_retry_wait_sec: u32,
    #[getset(get = "pub", set = "pub")]
    white_list_api: Vec<String>,
}

#[derive(Getters, Setters, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfig {
    #[getset(get = "pub", set = "pub")]
    url: String,
    #[getset(get = "pub", set = "pub")]
    logic_column: String,
    #[getset(get = "pub", set = "pub")]
    logic_un_deleted: u8,
    #[getset(get = "pub", set = "pub")]
    logic_deleted: u8,
}

#[derive(Getters, Setters, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogConfig {
    #[getset(get = "pub", set = "pub")]
    dir: String,
    #[getset(get = "pub", set = "pub")]
    log_temp_size: String,
    #[getset(get = "pub", set = "pub")]
    log_rolling_type: String,
    #[getset(get = "pub", set = "pub")]
    log_level: String,
    #[getset(get = "pub", set = "pub")]
    log_chan_len: usize,
}

#[derive(Getters, Setters, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheConfig {
    #[getset(get = "pub", set = "pub")]
    cache_type: String,
    #[getset(get = "pub", set = "pub")]
    redis_url: Option<String>,
}
