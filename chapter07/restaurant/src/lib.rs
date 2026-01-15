// [Module Declaration] 외부 파일로 분리된 모듈들을 선언
// 컴파일러에게 src/front_of_house.rs와 src/back_of_house.rs를 찾도록 알림
mod front_of_house;
mod back_of_house;

// [Re-exporting] 관용적인 API 설계
// front_of_house::hosting 모듈 use + pub 
pub use crate::front_of_house::hosting;

// [Path - super] back_of_house에서 참조할 함수
fn deliver_order() {
    println!("음식을 배달합니다.");
}

pub fn eat_at_restaurant() {
    // [Absolute Path] 크레이트 루트부터 시작하는 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // [Relative Path] 현재 위치에서 시작하는 상대 경로
    front_of_house::hosting::add_to_waitlist();

    // [Struct] 공개된 연관 함수(summer)를 통해 구조체 생성
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    meal.toast = String::from("통밀빵"); // pub 필드는 수정 가능
}