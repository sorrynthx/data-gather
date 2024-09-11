mod handlers; // 핸들러 모듈 선언 (라우트 처리 함수들을 포함)
mod routes;   // 라우트 모듈 선언 (라우트 설정 및 정의)
mod state;    // 상태 모듈 선언 (애플리케이션 상태 및 공유 데이터 관리)
mod utils;    // 유틸리티 모듈 선언 (공용 함수, 환경설정 로드 등)

use actix_web::{web, App, HttpServer};
use dotenv::dotenv; // .env 파일을 로드하기 위해 사용
use routes::configure_routes; // 라우트 설정 함수 임포트

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // .env 파일 로드 - 이 라인을 추가하여 환경 변수를 가져옵니다.
    println!("Start RUST Program"); // 프로그램 시작 로그 출력

    // 애플리케이션 상태 초기화
    // state::initialize_app_state() 함수를 호출하여 초기 상태를 설정하고, 
    // 이 상태는 AppState 구조체로 정의되어 있으며 다중 스레드에서 안전하게 상태를 공유하기 위해 사용됩니다.
    let app_state = web::Data::new(state::initialize_app_state());

    // HTTP 서버 시작
    // HttpServer::new 함수는 새로운 HTTP 서버 인스턴스를 생성합니다.
    // move || 클로저를 사용하여 서버가 실행될 때 필요한 설정을 정의합니다.
    HttpServer::new(move || {
        App::new()
            // app_data를 통해 초기화된 애플리케이션 상태를 서버에 전달합니다.
            .app_data(app_state.clone())
            // configure_routes 함수를 호출하여 서버의 라우트를 설정합니다.
            .configure(configure_routes) // 라우트 모듈에서 정의된 라우트들을 설정
    })
    // 서버를 localhost(127.0.0.1)의 포트 8080에 바인딩
    .bind("127.0.0.1:8080")?
    // 서버 실행
    .run()
    // 비동기 실행 완료 대기
    .await
}
