use actix_web::{web, App,HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count:Mutex::new(0),
    });  //初始化一个AppState
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
    };   //将share_data注册到web应用，这样就可以向handler输入数据了，然后配置路由

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await //创建webserver，然后把web应用传进去
}