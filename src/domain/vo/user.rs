#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    // pub login_check: Option<LoginCheck>,
    pub role_id: Option<String>,
    pub state: Option<i32>,
}