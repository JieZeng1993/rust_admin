use poem_openapi::{
    ApiResponse,
    Object,
    OpenApi,
    OpenApiService, param::Path, payload::Json, Tags, types::{Email, Password},
};

use crate::domain::vo::RespVO;
use crate::error::AppResult;

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User,
}

pub mod sys_user_rest;


#[derive(ApiResponse)]
enum CommonResponse<T>
    where
        T: Sync + Send + Clone + poem_openapi::types::Type + poem_openapi::types::ParseFromJSON + poem_openapi::types::ToJSON,
{
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<RespVO<T>>),
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InnerError,
}

impl<T> CommonResponse<T>
    where
        T: Sync + Send + Clone + poem_openapi::types::Type + poem_openapi::types::ParseFromJSON + poem_openapi::types::ToJSON
{
    pub fn deal_query(data_option_result: AppResult<Option<T>>) -> CommonResponse<T> {
        match data_option_result {
            Ok(data_option) => match data_option {
                Some(data) => CommonResponse::Ok(Json(RespVO::from(data))),
                None => CommonResponse::NotFound,
            },
            Err(_) => {
                log::error!("query failed");
                CommonResponse::InnerError
            }
        }
    }

    #[allow(dead_code)]
    pub fn deal_execute(data_option_result: AppResult<Option<T>>) -> CommonResponse<T> {
        match data_option_result {
            Ok(data_option) => match data_option {
                Some(data) => CommonResponse::Ok(Json(RespVO::from(data))),
                None => CommonResponse::Ok(Json(RespVO::no_data())),
            },
            Err(_) => {
                log::error!("execute failed");
                CommonResponse::InnerError
            }
        }
    }

    pub fn deal_result(data_result: AppResult<T>) -> CommonResponse<T> {
        match data_result {
            Ok(data) => CommonResponse::Ok(Json(RespVO::from(data))),
            Err(_) => {
                log::error!("execute failed");
                CommonResponse::InnerError
            }
        }
    }

    pub fn deal_resp_result(data_result: AppResult<RespVO<T>>) -> CommonResponse<T> {
        match data_result {
            Ok(data) => CommonResponse::Ok(Json(data)),
            Err(_) => {
                log::error!("execute failed");
                CommonResponse::InnerError
            }
        }
    }
}

impl CommonResponse<String> {
    pub fn deal_result_str(data_result: AppResult<&str>) -> CommonResponse<String> {
        match data_result {
            Ok(data) => CommonResponse::Ok(Json(RespVO::from(data.to_string()))),
            Err(err) => {
                log::error!("error info:{}",err);
                CommonResponse::Ok(Json(RespVO::from_error_code(err)))
            }
        }
    }
}
