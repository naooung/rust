// [Super] super를 사용하여 부모 모듈(lib.rs)의 함수에 접근
fn fix_incorrect_order() {
    cook_order();
    super::deliver_order(); 
}

fn cook_order() {}

// [Public Struct] 구조체는 공개되어도 필드는 기본적으로 비공개
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    // 비공개 필드가 있는 구조체는 객체를 생성해줄 pub 연관 함수 필요
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("복숭아"),
        }
    }
}

// [Public Enum] 
pub enum Appetizer {
    Soup,
    Salad,
}