extern crate rust_admin;

use poem::{Endpoint, EndpointExt, Route, Server};
use poem::listener::TcpListener;
use poem_openapi::OpenApiService;

use rust_admin::config::application_config::APPLICATION_CONFIG;
use rust_admin::config::session_context;
use rust_admin::config::session_context::{Session, SESSION_CONTEXT};
use rust_admin::context::{CONTEXT, ServerContext};
use rust_admin::rest::sys_user_rest::SysUserRest;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //加载配置
    println!("[{}] config read finish, log init", APPLICATION_CONFIG.app().name());
    //日志初始化
    rust_admin::config::log::init_log();
    log::error!("log init finish, app booting");

    //初始化Context
    log::info!("[{}] app {:?}",APPLICATION_CONFIG.app().name(),CONTEXT.state());

    CONTEXT.init_pool(APPLICATION_CONFIG.database()).await;

    let api = OpenApiService::new((SysUserRest), "api", "1.1")
        .server("http://localhost:3001/api");
    let swagger_ui = api.swagger_ui();

    // let tracer = init_tracer();

    log::info!("[{}] app start listen",APPLICATION_CONFIG.app().name());

    // SESSION_CONTEXT.scope(1, async move{
    //     let session =  SESSION_CONTEXT.get();
    //     return session;
    // }).await?;


    Server::new(TcpListener::bind("127.0.0.1:3001"))
        .run(
            Route::new()
                .nest("/api", api)
                // .at("/ws/:name", get(ws.data(tokio::sync::broadcast::channel::<String>(32).0)))
                .nest("/swagger_ui", swagger_ui)
                .around(|ep,req| async move{

                    let v = Some(Session{
                        id: Some(1),
                        name: "曾杰".to_string(),
                    });

                    let resp;

                    if v.is_some(){
                        resp = SESSION_CONTEXT.scope(v.unwrap(), async move{
                            ep.get_response(req).await
                        }).await;
                    }else {
                        resp = ep.get_response(req).await;
                    }

                    Ok(resp)
                })
            // .with(HeaderAuth::new()),
            // .data(tracer.clone())
            // .with(OpenTelemetryMetrics::new())
            // .with(OpenTelemetryTracing::new(tracer))
        )
        .await
}
// #[handler]
// fn ws(
//     Path(name): Path<String>,
//     ws: WebSocket,
//     sender: Data<&tokio::sync::broadcast::Sender<String>>,
// ) -> impl IntoResponse {
//     log!(
//         Level::Info,
//         "ws connect name:{name}"
//     );
//
//     match &name as &str {
//         "redisCli" => {
//             SERVICE_CONTEXT.redis_cli_service.deal(ws, sender)
//         }
//         _ => {
//             panic!("未实现的方法.{}", name)
//         }
//     }
// }