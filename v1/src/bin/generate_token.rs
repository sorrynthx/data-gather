// src/bin/generate_token.rs

// 필요한 크레이트(라이브러리) 임포트
use aes_gcm::aead::{Aead, KeyInit}; // 암호화/복호화를 위해 필요
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-256-GCM 암호화 구현체
use chrono::{DateTime, Duration, TimeZone, Utc}; // 날짜/시간 처리를 위한 크레이트
use serde::{Deserialize, Serialize}; // JSON 직렬화/역직렬화를 위해 필요
use rand::RngCore; // 난수 Nonce 값을 생성하기 위해 필요
use base64::engine::general_purpose::STANDARD; // Base64 인코딩/디코딩 엔진
use base64::Engine; // Base64 인코딩과 디코딩 메서드를 사용하기 위한 엔진 임포트
use std::env; // 명령줄 인자 처리용

// 토큰의 페이로드 구조체 정의
#[derive(Debug, Serialize, Deserialize)]
struct TokenPayload {
    exp: i64, // 만료 시간 (타임스탬프 형식)
    sub: String, // 식별 정보 (예: 버전 정보, 사용자 정보 등)
}

// 만료 시간 타임스탬프를 사람이 읽을 수 있는 날짜 형식으로 변환하는 함수
fn format_timestamp_to_date(exp: i64) -> String {
    // 타임스탬프를 날짜/시간 형식으로 변환 (초 단위 정확도)
    let datetime: DateTime<Utc> = Utc.timestamp_opt(exp, 0)
        .single() // Option<DateTime<Utc>>를 반환
        .expect("Invalid or ambiguous timestamp"); // 타임스탬프가 유효하지 않거나 애매할 때 오류 발생
    
    datetime.format("%Y-%m-%d %H:%M:%S").to_string() // 포맷팅하여 문자열로 변환
}

// 토큰을 생성, 암호화하여 암호화된 문자열로 반환하는 함수
fn generate_token(secret_key: &[u8; 32], days_valid: i64) -> String {
    // 현재 시간에서 유효 기간(days_valid)을 더해 만료 시간을 설정
    let expiration_date = Utc::now() + Duration::days(days_valid); 
    let payload = TokenPayload {
        exp: expiration_date.timestamp(), // 만료 시간을 타임스탬프로 변환
        sub: "collector-v1".to_string(), // 식별 정보를 설정
    };

    // 페이로드를 JSON 문자열로 직렬화
    let serialized_payload = serde_json::to_string(&payload).expect("Failed to serialize payload");

    // AES-256-GCM을 사용하여 직렬화된 페이로드를 암호화
    let key = Key::<Aes256Gcm>::from_slice(secret_key); // 암호화 키 생성
    let cipher = Aes256Gcm::new(key); // 암호화 객체 초기화

    // 랜덤 Nonce (12 바이트) 생성
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce); // 난수 생성기로 Nonce 값 채우기

    // 페이로드 암호화
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), serialized_payload.as_bytes())
        .expect("Encryption failed"); // 암호화 실패 시 오류 발생

    // Nonce와 암호문을 Base64로 인코딩하여 하나의 문자열로 반환
    let mut encrypted_data = nonce.to_vec(); // Nonce를 벡터로 변환
    encrypted_data.extend_from_slice(&ciphertext); // 암호문을 추가하여 데이터 확장
    STANDARD.encode(encrypted_data) // Base64 문자열로 반환
}

// 암호화된 토큰 문자열을 복호화하고 내용을 검증하는 함수
fn decrypt_token(encrypted_token: &str, secret_key: &[u8; 32]) -> Option<TokenPayload> {
    // Base64 디코딩을 통해 암호화된 데이터를 복원
    let encrypted_data = STANDARD.decode(encrypted_token).expect("Failed to decode base64 token"); 

    // Nonce와 암호문으로 나누기 (Nonce: 처음 12바이트)
    let (nonce, ciphertext) = encrypted_data.split_at(12);

    let key = Key::<Aes256Gcm>::from_slice(secret_key); // 복호화 키 생성
    let cipher = Aes256Gcm::new(key); // 복호화 객체 초기화

    // 암호문 복호화
    let decrypted_data = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .ok()?; // 복호화 실패 시 None 반환

    // 복호화된 데이터를 TokenPayload 구조체로 역직렬화
    let payload: TokenPayload =
        serde_json::from_slice(&decrypted_data).expect("Failed to deserialize token");
    Some(payload) // 복호화 및 역직렬화 성공 시 페이로드 반환
}

fn main() {
    // 명령줄 인자로부터 비밀 키와 유효 기간을 가져옴
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        // 인자 부족 시 사용법 안내 메시지 출력
        println!("Usage: cargo run --bin generate_token <secret_key> <days_valid>");
        return;
    }

    // 비밀 키가 정확히 32 바이트인지 확인
    let secret_key = args[1].as_bytes();
    if secret_key.len() != 32 {
        // 비밀 키가 32바이트가 아니면 오류 메시지 출력
        println!("Error: The secret key must be exactly 32 characters long.");
        return;
    }

    let mut key = [0u8; 32]; // 32바이트 배열 초기화
    key.copy_from_slice(secret_key); // 비밀 키를 배열에 복사

    // 유효 기간을 정수로 파싱
    let days_valid: i64 = args[2].parse().expect("Please enter a valid number of days"); 

    // 토큰 생성 및 암호화
    let encrypted_token = generate_token(&key, days_valid);
    println!("Generated Encrypted Token: {}", encrypted_token); // 생성된 암호화된 토큰 출력

    // 예시: 토큰을 복호화하여 페이로드 확인
    if let Some(payload) = decrypt_token(&encrypted_token, &key) {
        // 타임스탬프를 읽을 수 있는 날짜 형식으로 변환
        let formatted_date = format_timestamp_to_date(payload.exp);
        println!(
            "Decrypted Token Payload: {{ exp: {}, sub: \"{}\", date: {} }}",
            payload.exp, payload.sub, formatted_date
        ); // 페이로드와 변환된 날짜 출력
    } else {
        // 복호화 실패 시 메시지 출력
        println!("Failed to decrypt the token.");
    }
}
