use std::fs; // 파일을 읽기 위해 사용
use std::path::Path; // 파일 경로 처리를 위해 사용
use chrono::Utc; // 현재 시간을 타임스탬프 형식으로 가져오기 위해 사용
use aes_gcm::aead::{Aead, KeyInit}; // AES-GCM 암호화를 위한 기본 구성
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-256-GCM 암호화 구현체
use base64::engine::general_purpose::STANDARD; // Base64 인코딩/디코딩 엔진
use base64::Engine; // Base64 엔진의 인코딩과 디코딩 메서드를 사용하기 위한 엔진 임포트
use serde::{Deserialize, Serialize}; // JSON 직렬화/역직렬화를 위해 필요

// 토큰의 페이로드 구조체 정의
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub exp: i64, // 만료 시간 (타임스탬프 형식)
    pub sub: String, // 식별 정보 (예: 버전 정보, 사용자 정보 등)
}

// 키를 읽고 유효성을 검증하는 함수
// 구성 파일에서 키를 읽고, 키의 유효성을 검증하여 만료 여부를 설정합니다.
pub fn read_and_validate_key(config_path: &Path, secret_key: &[u8; 32], is_expired: &mut bool) -> Result<(), String> {
    // 구성 파일을 읽음
    let contents = fs::read_to_string(config_path).map_err(|_| "Failed to read configuration file")?;
    
    // 'key=' 다음에 오는 값을 파싱
    if let Some(line) = contents.lines().find(|line| line.starts_with("key=")) {
        let encrypted_token = line.trim_start_matches("key=").trim(); // 키 값만 추출
        validate_key(encrypted_token, secret_key, is_expired) // 키 검증 함수 호출
    } else {
        Err("Key not found in configuration file".to_string()) // 키가 없으면 오류 반환
    }
}

// 키의 유효성을 검증하고 만료 시간을 확인하는 함수
// 토큰을 복호화하여 유효성 검증을 수행하고, 만료 여부를 설정합니다.
pub fn validate_key(encrypted_token: &str, secret_key: &[u8; 32], is_expired: &mut bool) -> Result<(), String> {
    // 토큰을 복호화하여 유효성을 검증
    if let Some(payload) = decrypt_token(encrypted_token, secret_key) {
        let current_timestamp = Utc::now().timestamp(); // 현재 시간을 타임스탬프 형식으로 가져오기
        if payload.exp > current_timestamp {
            *is_expired = false; // 만료 시간이 현재 시간보다 크면 유효
            Ok(())
        } else {
            *is_expired = true; // 만료 시간이 현재 시간보다 작으면 만료됨
            Ok(())
        }
    } else {
        Err("Failed to decrypt token or invalid token.".to_string()) // 복호화 실패 시 오류 반환
    }
}

// 암호화된 토큰 문자열을 복호화하고 내용을 검증하는 함수
// AES-GCM 암호화 방식으로 암호화된 토큰을 복호화하고, 페이로드를 검증합니다.
pub fn decrypt_token(encrypted_token: &str, secret_key: &[u8; 32]) -> Option<TokenPayload> {
    // Base64 디코딩을 통해 암호화된 데이터를 복원
    let encrypted_data = STANDARD.decode(encrypted_token).ok()?; 
    let (nonce, ciphertext) = encrypted_data.split_at(12); // Nonce와 암호문으로 나누기 (Nonce: 처음 12바이트)

    let key = Key::<Aes256Gcm>::from_slice(secret_key); // 복호화 키 생성
    let cipher = Aes256Gcm::new(key); // 복호화 객체 초기화

    // 암호문 복호화
    let decrypted_data = cipher.decrypt(Nonce::from_slice(nonce), ciphertext).ok()?; // 복호화 실패 시 None 반환
    let payload: TokenPayload = serde_json::from_slice(&decrypted_data).ok()?; // 복호화된 데이터를 TokenPayload 구조체로 역직렬화
    Some(payload) // 복호화 및 역직렬화 성공 시 페이로드 반환
}
