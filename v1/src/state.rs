use std::sync::Mutex; // 다중 스레드 환경에서 안전하게 데이터에 접근하기 위해 Mutex를 사용
use std::path::Path; // 파일 경로 처리를 위해 사용
use crate::utils::read_and_validate_key; // utils 모듈에서 토큰 검증 함수를 임포트

// AppState 구조체 정의
// 애플리케이션의 상태를 공유하기 위해 사용되는 구조체로,
// 여러 핸들러들이 이 상태를 참조하여 동작할 수 있습니다.
pub struct AppState {
    pub is_expired: Mutex<bool>, // 토큰의 만료 여부를 저장하는 Mutex로 보호된 bool 값
}

// 애플리케이션 상태를 초기화하는 함수
// 서버가 시작될 때 호출되어 AppState의 초기 상태를 설정합니다.
// 이 함수는 AppState를 생성하고, 구성 파일에서 키를 읽어 초기 상태를 설정합니다.
pub fn initialize_app_state() -> AppState {
    // 만료 여부를 저장할 변수를 초기화합니다.
    let mut is_expired = false;

    // 환경 변수에서 구성 파일 경로와 비밀 키를 가져옵니다.
    let config_path_str = std::env::var("CONFIG_PATH").expect("CONFIG_PATH not set"); // CONFIG_PATH의 값을 String으로 저장
    let config_path = Path::new(&config_path_str); // String의 참조를 사용하여 Path 생성

    let secret_key_str = std::env::var("SECRET_KEY").expect("SECRET_KEY not set"); // 비밀 키를 환경 변수에서 가져옵니다.
    let secret_key = secret_key_str.as_bytes(); // 비밀 키를 바이트 배열로 변환합니다.

    if secret_key.len() != 32 {
        eprintln!("Error: SECRET_KEY must be exactly 32 bytes long."); // 키 길이 오류 처리
    } else {
        let mut key = [0u8; 32];
        key.copy_from_slice(secret_key);

        // 토큰의 유효성을 검증하여 만료 여부를 설정합니다.
        if let Err(err) = read_and_validate_key(config_path, &key, &mut is_expired) {
            eprintln!("Error validating token: {}", err); // 검증 오류가 발생할 경우 오류 메시지 출력
        }
    }

    // AppState를 생성하여 반환합니다.
    AppState {
        is_expired: Mutex::new(is_expired), // 초기 만료 상태를 설정
    }
}
