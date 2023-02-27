extern crate rust_admin;

use rust_admin::context::CONTEXT;

fn main() {
    log::info!("[{}]app booted",CONTEXT.config().app().name());
    log::error!("log init finish2, app booting");
    log::error!("log init finish3, app booting");
    log::error!("log init finish4, app booting");
    log::error!("log init finish5, app booting");
    log::error!("log init finish6, app booting");
    log::error!("log init finish7, app booting");

    println!("Hello, world!");
}
