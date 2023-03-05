use poem_openapi::Object;

#[derive(Object, Serialize, Deserialize, Clone, Debug)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    // pub login_check: Option<LoginCheck>,
    pub role_id: Option<String>,
    pub state: Option<i32>,
}

#[derive(Object, Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdateDTO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub state: Option<i32>,
    // pub login_check: Option<LoginCheck>,
    pub role_id: Option<String>,
}