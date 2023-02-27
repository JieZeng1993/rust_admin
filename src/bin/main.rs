extern crate rust_admin;

use rust_admin::context::CONTEXT;

fn main() {
    println!("[{}] config read finish, log init", CONTEXT.config().app().name());
    rust_admin::config::log::init_log();
    while true {

        log::error!("log init finish, app booting");
    }
    println!("Hello, world!");
}
