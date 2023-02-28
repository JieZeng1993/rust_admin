use getset::{Getters, Setters};
use once_cell::sync::Lazy;

use crate::config::application_config::APPLICATION_CONFIG;
use crate::service::cache_service::CacheService;

#[derive(Debug)]
pub enum ServerState {
    Booting,
    Booted,
    Shutting,
    Shutdown,
}

#[derive(Getters, Setters)]
pub struct ServerContext {
    #[getset(get = "pub", set = "pub")]
    state: ServerState,
    // pub rb: Rbatis,
    #[getset(get = "pub")]
    cache_service: CacheService,
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
    //init
    fn default() -> Self {
        ServerContext {
            state: ServerState::Booting,
            // rb: crate::domain::init_rbatis(&config),
            cache_service: CacheService::new(APPLICATION_CONFIG.cache()).unwrap(),
            // sys_res_service: SysResService {},
            // sys_user_service: SysUserService {},
            // sys_role_service: SysRoleService {},
            // sys_role_res_service: SysRoleResService {},
            // sys_user_role_service: SysUserRoleService {},
            // sys_dict_service: SysDictService {},
            // sys_auth_service: SysAuthService {},
            // sys_trash_service: SysTrashService {},
        }
    }
}


/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServerContext> = Lazy::new(|| ServerContext::default());