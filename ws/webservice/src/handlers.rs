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

use super::models::Course;
use chrono::Utc;

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state:web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");
    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id) //过滤器取出id相等的course
        .collect::<Vec<Course>>() //将它转换为vector
        .len(); //然后计算长度
    let new_course = Course {
        teacher_id: new_course.teacher_id, //创建一个新的课程
        id: Some(course_count + 1), //id为课程数+1
        name: new_course.name.clone(), //名称就是传进来的名称
        time: Some(Utc::now().naive_utc()), //时间调用Utc这个包
    };
    app_state.courses.lock().unwrap().push(new_course); //将这个包传入到app中
    HttpResponse::Ok().json("Course added") 
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test] //测试函数一般上面写一个test就可以了，但是由于是异步的所以要加上actix_rt
    async fn post_course_test() {
        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: None,
            time: None,
        });  //创建一个课程
        let app_state: web::Data<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        }); //创建一个应用程序
        let resp = new_course(course, app_state).await;  //模拟一个请求
        assert_eq!(resp.status(), StatusCode::OK);
    }
}