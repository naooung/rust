// [Public Struct] 구조체는 pub을 붙여도 필드는 각각 pub을 붙여야 공개됨
#[derive(Debug)]
pub struct Asparagus {}

// [Public Enum] 열거형은 pub 선언 시 내부 배리언트(Onion, Tomato)가 모두 자동 공개됨
pub enum Soup {
    Onion,
    Tomato,
}