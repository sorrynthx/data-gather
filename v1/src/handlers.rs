use actix_files::NamedFile;                                 // Actix Web의 NamedFile을 사용하여 정적 파일을 제공
use actix_web::{HttpRequest, HttpResponse, Responder, web}; // 필요한 Actix Web 타입 임포트
use std::path::PathBuf;                                     // 파일 경로 처리를 위해 PathBuf 사용
use crate::state::AppState;                                 // 애플리케이션의 상태를 공유하기 위해 state 모듈의 AppState를 임포트

// 루트 핸들러 - index.html 파일을 제공
// 사용자가 "/" 경로로 접근할 때 index.html 파일을 반환합니다.
pub async fn index_handler(_req: HttpRequest) -> impl Responder {
    
    // index.html 파일의 경로를 지정
    let path: PathBuf = "./static/index.html".into();

    // 파일을 열어 반환하거나, 오류 발생 시 404 응답을 반환합니다.
    NamedFile::open(path).map_err(|e| actix_web::error::ErrorNotFound(e))
}

// 상태 핸들러 - 애플리케이션의 상태를 확인
// "/status" 경로로 접근 시 현재 토큰의 만료 여부를 JSON 형식으로 반환합니다.
pub async fn status_handler(data: web::Data<AppState>) -> impl Responder {
    
    // 상태에서 만료 여부를 읽어옴
    let is_expired = *data.is_expired.lock().unwrap();
    // 만료 상태를 JSON으로 응답
    HttpResponse::Ok().json(serde_json::json!({ "isExpired": is_expired }))
}

// "/test1" 경로 핸들러 - 간단한 텍스트 응답
// "/test1" 경로로 접근할 때 간단한 환영 메시지를 반환합니다.
pub async fn test1_handler() -> impl Responder {
    // 단순 텍스트 응답 반환
    HttpResponse::Ok().body("Welcome to /test1 page!") // 단순 텍스트 응답 반환
}
