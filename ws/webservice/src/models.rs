use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)] //Deserialize,Serialize是反序列化和序列化，他们两个来自serde
pub struct Course {
    pub teacher_id:usize,
    pub id: Option<usize>, //可为空，因为在post的时候这个值是没有的。
    pub name:String,
    pub time:Option<NaiveDateTime>, //可为空，NaiveDateTime来自chrono，是一个日期时间类型。
}
 
impl From<web::Json<Course>> for Course {  //可以将json格式的数据转化为Course这个类型
    fn from(course: web::Json<Course>) -> Self {
        Course {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time,
        }
    }
}