use poem_openapi::Object;
use time::OffsetDateTime;
use crate::tables::SysUser;
use crate::util::password_encoder::PasswordEncoder;

#[derive(Object, Serialize, Deserialize, Clone, Debug)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    // pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub create_date: Option<OffsetDateTime>,
}

impl From<UserAddDTO> for SysUser {
    fn from(arg: UserAddDTO) -> Self {
        SysUser {
            id: None,
            account: arg.account.clone(),
            password: PasswordEncoder::encode(&arg.password.unwrap_or_default()).into(),
            name: arg.name.clone(),
            state: Some(arg.state.unwrap_or(1)),
            del: 0.into(),
            create_date: arg.create_date,
        }
    }
}


#[derive(Object, Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdateDTO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub state: Option<i32>,
    // pub login_check: Option<LoginCheck>,
}