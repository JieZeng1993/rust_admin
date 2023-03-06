use crate::domain::dto::user::UserAddDTO;
use crate::error::{AppError, AppResult};
use crate::pool;
use crate::tables::SysUser;

const REDIS_KEY_RETRY: &'static str = "login:login_retry";

///Background User Service
pub struct SysUserService {}

impl SysUserService {
    pub async fn add(&self, mut arg: UserAddDTO) -> AppResult<u64> {
        if arg.account.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.name.is_none()
            || arg.name.as_ref().unwrap().is_empty()
        {
            return Err(AppError::from("用户名和姓名不能为空!"));
        }
        let old_user = self
            .find_by_account(arg.account.as_ref().unwrap())
            .await?;
        if old_user.is_some() {
            return Err(AppError::from(format!(
                "用户账户:{}已存在!",
                arg.account.as_ref().unwrap()
            )));
        }
        let mut password = arg.password.as_deref().unwrap_or_default().to_string();
        if password.is_empty() {
            //default password
            password = "123456".to_string();
        }
        arg.password = Some(password);
        let user = SysUser::from(arg);
        Ok(SysUser::insert(pool!(), &user).await?.rows_affected)
    }

    pub async fn find_by_account(&self, account: &str) -> AppResult<Option<SysUser>> {
        Ok(SysUser::select_by_column(pool!(), "account", account)
            .await?
            .into_iter()
            .next())
    }
}
