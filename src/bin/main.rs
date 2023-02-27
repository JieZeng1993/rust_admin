extern crate rust_admin;

use rust_admin::context::CONTEXT;

fn main() {
    println!("[{}] config read finish, log init", CONTEXT.config().app().name());
    rust_admin::config::log::init_log();
    log::error!("log init finish1, app booting");
    log::error!("log init finish2, app booting");
    log::error!("log init finish3, app booting");
    log::error!("log init finish4, app booting");
    log::error!("log init finish5, app booting");
    log::error!("log init finish6, app booting");
    log::error!("log init finish7, app booting");

    println!("Hello, world!");
}
