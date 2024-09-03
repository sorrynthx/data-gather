use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HTTP 서버를 시작합니다
    println!("Start RUST Program"); // 프로그램 시작을 알리는 출력문
    HttpServer::new(|| {
        App::new()
            .service(
                // 정적 파일을 제공하는 서비스 설정 (필요에 따라 경로 조정)
                Files::new("/", "./static") // 현재 디렉토리의 ./static 폴더에서 파일을 제공
                    .index_file("index.html") // index.html을 기본 파일로 설정하여 루트 경로에서 보여줌
            )
            .default_service(
                // 기본 서비스 설정: 다른 라우트가 없는 경우 404 페이지를 반환
                web::route().to(|| async {
                    HttpResponse::NotFound().body("404 Not Found") // 경로를 찾을 수 없는 경우 404 메시지 반환
                }),
            )
    })
    .bind("127.0.0.1:8080")? // 서버를 localhost:8080에 바인딩 (접속할 주소와 포트 설정)
    .run() // 서버 실행
    .await // 비동기 실행 대기
}
