use std::time::Duration;

use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{FileSplitAppender, RollingType};

use crate::context::CONTEXT;

pub fn init_log() {
    let log_config = &CONTEXT.config().log();
    let app_config = &CONTEXT.config().app();
    //create log dir
    let _ = std::fs::create_dir_all(format!("{}/{}", log_config.dir(), app_config.uk()));
    //init fast log
    let mut cfg = Config::new()
        .level(str_to_log_level(log_config.log_level()))
        .custom(FileSplitAppender::new(&format!("{}/{}/{}", log_config.dir(), app_config.uk(), app_config.name()),
                                       str_to_temp_size(log_config.log_temp_size()),
                                       str_to_rolling(log_config.log_rolling_type()),
                                       Box::new(fast_log::plugin::packer::LogPacker {}),
        ));
    if *app_config.debug() {
        cfg = cfg.console();
    }
    cfg = cfg.chan_len(Some(*log_config.log_chan_len()));
    let _ = fast_log::init(cfg);
    if !*app_config.debug() {
        println!("[{}] release_mode is up! [file_log] open,[console_log] disabled!", app_config.name());
    }
}

fn str_to_temp_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn str_to_rolling(arg: &str) -> RollingType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}

fn str_to_log_level(arg: &str) -> log::LevelFilter {
    return match arg {
        "off" => log::LevelFilter::Off,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        "trace" => log::LevelFilter::Trace,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        _ => log::LevelFilter::Info,
    };
}
