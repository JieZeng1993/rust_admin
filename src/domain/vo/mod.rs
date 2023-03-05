use poem_openapi::{
    ApiResponse,
    Object,
    OpenApi,
    OpenApiService, param::Path, payload::Json, Tags, types::{Email, Password},
};

use crate::error::AppError;

pub mod user;

pub const CODE_COMMON_FAIL: &str = "COMMON_FAIL";

/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// #[oai(rename_all = "camelCase")]
pub struct RespVO<T>
    where
        T: Sync + Send + Clone + poem_openapi::types::Type + poem_openapi::types::ParseFromJSON + poem_openapi::types::ToJSON,
{
    pub success: bool,
    pub msg: Option<String>,
    //error display type： 0 silent; 1 message.warn; 2 message.error; 4 notification; 9 page
    pub show_type: Option<i16>,
    pub data: Option<T>,
    pub error_code: Option<String>,
    pub current: Option<u64>,
    pub page_size: Option<u64>,
    pub total: Option<u64>,
}

impl<T> RespVO<T>
    where
        T: Sync + Send + Clone + poem_openapi::types::Type + poem_openapi::types::ParseFromJSON + poem_openapi::types::ToJSON,
{
    ///通过引用实例化
    pub fn from_refer(arg: &T) -> Self {
        Self {
            success: true,
            msg: None,
            error_code: None,
            data: Some(arg.clone()),
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    ///通过转移所有权构造
    pub fn from(arg: T) -> Self {
        Self {
            success: true,
            msg: None,
            error_code: None,
            data: Some(arg),
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    pub fn success_msg(msg: String) -> Self {
        Self {
            success: true,
            msg: Some(msg),
            error_code: None,
            data: None,
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    pub fn no_data() -> Self {
        Self {
            success: true,
            msg: None,
            error_code: None,
            data: None,
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    pub fn from_error(code: &str, arg: &AppError) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = CODE_COMMON_FAIL.to_string();
        }
        Self {
            success: false,
            msg: Some(arg.to_string()),
            error_code: Some(code_str),
            data: None,
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    pub fn from_error_code(arg: AppError) -> Self {
        let code_str = arg.to_string();
        Self {
            success: false,
            //这个地方后续需要国际化
            msg: Some(code_str.clone()),
            error_code: Some(code_str),
            data: None,
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }

    pub fn from_error_info(code: &str, info: &str) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = CODE_COMMON_FAIL.to_string();
        }
        Self {
            success: false,
            msg: Some(info.to_string()),
            error_code: Some(code_str),
            data: None,
            current: None,
            page_size: None,
            total: None,
            show_type: None,
        }
    }
}