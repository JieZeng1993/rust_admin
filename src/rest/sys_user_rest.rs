use poem_openapi::{OpenApi, payload::Json, Tags};
use crate::context::CONTEXT;

use crate::domain::vo::RespVO;
use crate::domain::vo::user::{UserAddDTO, UserUpdateDTO};
use crate::rest::ApiTags;
use crate::rest::CommonResponse;

pub struct SysUserRest;


#[OpenApi]
impl SysUserRest {
    // /// 登录
    // #[oai(path = "/user/login", method = "post", tag = "ApiTags::User")]
    // async fn login(&self, login_dto: Json<UserLoginDto>) -> CommonResponse<LoginVo> {
    //     let user = SERVICE_CONTEXT
    //         .user_service
    //         .login(
    //             login_dto.name.as_ref().unwrap(),
    //             login_dto.password.as_ref().unwrap(),
    //         )
    //         .await;
    //     match user {
    //         Ok(data) =>  {
    //             CommonResponse::Ok(Json(RespVO::from(data)))
    //         },
    //         Err(err) => {
    //             CommonResponse::Ok(Json(RespVO::from_error_code(err)))
    //         }
    //     }
    // }

    /// 新增
    #[oai(path = "/user/add", method = "post", tag = "ApiTags::User")]
    async fn add(&self, add_dto: Json<UserAddDTO>) -> CommonResponse<String> {
        CONTEXT.sys_user_service().add(add_dto.0).await.unwrap();
        CommonResponse::Ok(Json(RespVO::from("ADD".to_string())))
    }

    /// 新增
    #[oai(path = "/user/update", method = "post", tag = "ApiTags::User")]
    async fn update(&self, update_dto: Json<UserUpdateDTO>) -> CommonResponse<String> {
        // let user = CONTEXT
        //     .user_service
        //     .login(
        //         login_dto.name.as_ref().unwrap(),
        //         login_dto.password.as_ref().unwrap(),
        //     )
        //     .await;
        // match user {
        //     Ok(data) =>  {
        //         CommonResponse::Ok(Json(RespVO::from(data)))
        //     },
        //     Err(err) => {
        //         CommonResponse::Ok(Json(RespVO::from_error_code(err)))
        //     }
        // }
        CommonResponse::Ok(Json(RespVO::from("update".to_string())))
    }
}

// pub async fn login(arg: web::Json<SignInDTO>) -> impl Responder {
//     log::info!("login:{:?}", arg.0);
//     let vo = CONTEXT.sys_user_service.sign_in(&arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }

// pub async fn info(req: HttpRequest) -> impl Responder {
//     let token = req.headers().get("access_token");
//     return match token {
//         Some(token) => {
//             let token = token.to_str().unwrap_or("");
//             let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
//             if token.is_err() {
//                 return RespVO::from_result(&token).resp_json();
//             }
//             let user_data = CONTEXT
//                 .sys_user_service
//                 .get_user_info_by_token(&token.unwrap())
//                 .await;
//             RespVO::from_result(&user_data).resp_json()
//         }
//         _ => RespVO::<String>::from_error_info("access_token is empty!", "").resp_json(),
//     };
// }

// pub async fn add(arg: web::Json<UserAddDTO>) -> impl Responder {
//     let vo = CONTEXT.sys_user_service.add(arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }
//
// pub async fn page(arg: web::Json<UserRolePageDTO>) -> impl Responder {
//     let vo = CONTEXT.sys_user_role_service.page(&arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }
//
// pub async fn detail(arg: web::Json<IdDTO>) -> impl Responder {
//     let vo = CONTEXT.sys_user_service.detail(&arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }
//
// pub async fn update(arg: web::Json<UserEditDTO>) -> impl Responder {
//     let vo = CONTEXT.sys_user_service.edit(arg.0).await;
//     return RespVO::from_result(&vo).resp_json();
// }
//
// pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
//     let vo = CONTEXT
//         .sys_user_service
//         .remove(&arg.0.id.unwrap_or_default())
//         .await;
//     return RespVO::from_result(&vo).resp_json();
// }
