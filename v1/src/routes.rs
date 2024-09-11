use actix_web::web; // Actix Web의 웹 모듈을 임포트하여 라우트 설정에 사용
use crate::handlers::{index_handler, status_handler, test1_handler}; // 핸들러 모듈에서 필요한 핸들러 함수들 임포트

// 라우트를 설정하는 함수
// 이 함수는 Actix Web의 ServiceConfig를 받아, 각 경로와 해당 경로에서 처리할 핸들러를 설정합니다.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // "/" 경로에 대한 설정
        // index_handler를 호출하여 index.html을 제공하는 역할을 합니다.
        .route("/", web::get().to(index_handler)) 

        // "/status" 경로에 대한 설정
        // status_handler를 호출하여 애플리케이션의 상태(토큰 만료 여부 등)를 JSON 형식으로 반환합니다.
        .route("/status", web::get().to(status_handler)) 

        // "/test1" 경로에 대한 설정
        // test1_handler를 호출하여 간단한 메시지를 반환하는 페이지를 제공합니다.
        .route("/test1", web::get().to(test1_handler)); 
}
