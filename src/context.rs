use getset::{Getters, Setters};
use once_cell::sync::Lazy;
use rbatis::Rbatis;

use crate::config::application_config::APPLICATION_CONFIG;
use crate::config::domain::DatabaseConfig;
use crate::service::cache_service::CacheService;
use crate::service::sys_user_service::SysUserService;

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
    #[getset(get = "pub", set = "pub")]
    rb: Rbatis,
    #[getset(get = "pub", set = "pub")]
    cache_service: CacheService,
    // pub sys_res_service: SysResService,
    #[getset(get = "pub", set = "pub")]
    sys_user_service: SysUserService,
    // pub sys_role_service: SysRoleService,
    // pub sys_role_res_service: SysRoleResService,
    // pub sys_user_role_service: SysUserRoleService,
    // pub sys_dict_service: SysDictService,
    // pub sys_auth_service: SysAuthService,
    // pub sys_trash_service: SysTrashService,
}

impl ServerContext {
    /// init database pool
    pub async fn init_pool(&self, database_config: &DatabaseConfig) {
        let database_url = database_config.url();
        log::info!(
            "rbatis pool init ({})...",
           database_url
        );
        let driver = rbdc_mysql::driver::MysqlDriver {};
        let driver_name = format!("{:?}", driver);
        self.rb
            .init(driver, database_url)
            .expect("rbatis pool init fail!");
        self.rb.acquire().await.expect(&format!(
            "rbatis connect database(driver={},url={}) fail",
            driver_name, database_url
        ));
        log::info!(
            "rbatis pool init success! pool state = {:?}",
            self.rb.get_pool().expect("pool not init!").status()
        );
    }
}


impl Default for ServerContext {
    //init
    fn default() -> Self {
        ServerContext {
            state: ServerState::Booting,
            // rb: crate::domain::init_rbatis(&config),
            rb: crate::mapper::init_rbatis(APPLICATION_CONFIG.app()),
            cache_service: CacheService::new(APPLICATION_CONFIG.cache()).unwrap(),
            // sys_res_service: SysResService {},
            sys_user_service: SysUserService {},
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