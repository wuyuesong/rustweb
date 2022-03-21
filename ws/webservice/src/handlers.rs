use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(
    app_state: web::Data<AppState>
) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap(); //调用之前必须lock，防止多个线程修改这个值
    let response = 
        format!("{} {} times", health_check_response,visit_count);
    *visit_count += 1;  //更行锁的值，锁释放的时间为走完这个handler
    HttpResponse::Ok().json(&response)
}