use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;

pub async fn index() -> &'static str {
    r"Casefeed [beta]

법원/헌법재판소 판례공보를 RSS 형태로 제공하는 서비스입니다.

※ RSS란? 웹사이트의 새로운 콘텐츠를 자동으로 받아볼 수 있게 해주는 형식입니다. RSS 리더 앱이나 프로그램을 통해 여러 사이트의 새 글을 한 곳에서 확인할 수 있습니다.

■ 사용 방법
아래 링크를 복사하여 사용하시는 RSS 리더에 추가해주세요.

- 대법원 판례공보: https://casefeed.kr/대법원/판례공보
- 헌법재판소 판례공보: https://casefeed.kr/헌법재판소/판례공보

■ 검색 설정 변경 (파라미터)
위 링크를 원하는 대로 조정하여 검색 결과를 맞춤 설정할 수 있습니다.

◎ 파라미터 사용 방법
1. 위 기본 링크 맨 뒤에 '?'를 입력합니다.
2. 원하는 파라미터와 값을 'parameter=value' 형태로 입력합니다.
3. 여러 파라미터를 사용할 경우 '&'로 구분합니다.

◎ 사용 가능한 파라미터
- length: 한 번에 검색할 판례의 개수 (0~256, 기본값: 40)
    예시: https://casefeed.kr/대법원/판례공보?length=100

- case-provider: 판례 정보를 제공받을 서비스 선택
    - casenote: 케이스노트 (기본값)
    - lbox: 엘박스
    - bigcase: 빅케이스
    예시: https://casefeed.kr/대법원/판례공보?case-provider=lbox

◎ 파라미터 조합 예시
https://casefeed.kr/대법원/판례공보?length=100&case-provider=lbox
"
}

pub async fn favicon() -> impl IntoResponse {
    let favicon_data = include_bytes!("../../static/favicon.ico");

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/x-icon"));

    (StatusCode::OK, headers, favicon_data.to_vec())
}
