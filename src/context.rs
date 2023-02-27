use getset::{Getters, Setters};
use once_cell::sync::Lazy;

use crate::config::application_config::ApplicationConfig;

#[derive(Getters, Setters)]
pub struct ServerContext {
    #[getset(get = "pub", set = "pub")]
    config: ApplicationConfig,
    // pub rb: Rbatis,
    // pub cache_service: CacheService,
    // pub sys_res_service: SysResService,
    // pub sys_user_service: SysUserService,
    // pub sys_role_service: SysRoleService,
    // pub sys_role_res_service: SysRoleResService,
    // pub sys_user_role_service: SysUserRoleService,
    // pub sys_dict_service: SysDictService,
    // pub sys_auth_service: SysAuthService,
    // pub sys_trash_service: SysTrashService,
}

impl Default for ServerContext {
    fn default() -> Self {
        //TODO 启动代码
        let config = ApplicationConfig::default();
        println!("[{}] config read finish, log init", config.app().name());
        //日志初始化
        crate::config::log::init_log();
        log::error!("log init finish1, app booting");

        ServerContext {
            // rb: crate::domain::init_rbatis(&config),
            // cache_service: CacheService::new(&config).unwrap(),
            // sys_res_service: SysResService {},
            // sys_user_service: SysUserService {},
            // sys_role_service: SysRoleService {},
            // sys_role_res_service: SysRoleResService {},
            // sys_user_role_service: SysUserRoleService {},
            // sys_dict_service: SysDictService {},
            // sys_auth_service: SysAuthService {},
            // sys_trash_service: SysTrashService {},
            config,
        }
    }
}


/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServerContext> = Lazy::new(|| ServerContext::default());