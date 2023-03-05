use std::borrow::Cow;

use poem_openapi::registry::MetaSchemaRef;
use poem_openapi::types::Type;
use rbatis::Rbatis;
use time::OffsetDateTime;

use crate::config::domain::AppConfig;

pub mod sys_user;


pub fn init_rbatis(app_config: &AppConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if rbatis.is_debug_mode() == false && *app_config.debug() {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 application.yml 中debug配置项为  debug: false"#
        );
    }
    return rbatis;
}
