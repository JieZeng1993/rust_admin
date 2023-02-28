extern crate rust_admin;

use rust_admin::config::application_config::APPLICATION_CONFIG;
use rust_admin::context::CONTEXT;

fn main() {
    //加载配置
    println!("[{}] config read finish, log init", APPLICATION_CONFIG.app().name());
    //日志初始化
    rust_admin::config::log::init_log();
    log::error!("log init finish, app booting");

    //初始化Context
    log::info!("[{}] app {:?}",APPLICATION_CONFIG.app().name(),CONTEXT.state());


    log::info!("Hello, world!");
}
