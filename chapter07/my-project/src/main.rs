// [use] 단축 경로 생성
use crate::garden::vegetables::Asparagus;

// [mod] 모듈 선언: 컴파일러에게 src/garden.rs 파일을 읽어오도록 지시
pub mod garden;

fn main() {
    let plant = Asparagus {}; // vegetables 모듈에 정의된 구조체 사용
    println!("I'm growing {plant:?}!");
}