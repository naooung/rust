// [crate root] 바이너리 크레이트의 시작점
// mod 선언을 통해 해당 이름의 파일(src/front_of_house.rs 등)을 모듈로 포함
mod front_of_house;
mod back_of_house;

// [Idiomatic use] 함수는 부모 모듈까지만 가져와서 로컬 정의가 아님을 명시
use crate::front_of_house::hosting;

// [Re-exporting] pub use를 통해 내부 구조를 몰라도 외부에서 접근 가능하도록 재내보내기
pub use crate::back_of_house::Asparagus;

fn main() {
    // use로 가져온 단축 경로 사용
    hosting::add_to_waitlist();
    
    let plant = Asparagus {};
    println!("준비된 식물: {:?}", plant);
}