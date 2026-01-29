use actix_web::{web, Responder};
use serde_json::Value;

use crate::domain::vo::RespVO;
use crate::service::deepseek_service;

pub async fn chat(arg: web::Json<Value>) -> impl Responder {
    let result = deepseek_service::chat(arg.into_inner()).await;
    RespVO::from_result(&result).resp_json()
}
