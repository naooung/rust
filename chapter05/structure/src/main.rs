// [STEP 1] 구조체 정의
// ---------------------------------------------------------
// 설명: {:?} 포맷으로 구조체를 출력하기 위해 Debug 트레이트를 자동으로 구현합니다.
// 학습 포인트: 기본 타입(u32 등)은 Display가 있지만, 구조체는 출력 방식이 모호하여
//            직접 Debug 기능 추가(opt-in) 필요
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// [STEP 2] 구현 블록 (impl) - 데이터와 행동의 분리
// ---------------------------------------------------------
// 설명: Rectangle 구조체와 관련된 모든 동작(생성자, 함수) 정의
impl Rectangle {
    // 2-1. 연관 함수 (Associated Function)
    // -----------------------------------------------------
    // 특징: self를 파라미터로 받지 않음
    // 용도: 주로 생성자(Constructor) 역할
    // 사용법: Rectangle::square(10);
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size, // 파라미터명과 필드명이 같으면 단축 구문 사용 가능 (인자로 width/height를 받을 때)
        }
    }

    // 2-2. 메서드 (Method)
    // -----------------------------------------------------
    // 특징: 첫 번째 파라미터가 항상 &self (인스턴스의 데이터에 접근)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 특징: 다른 인스턴스(other)를 파라미터로 받을 수 있음 (쉼표로 구분하며 파라미터의 별명 : 타입으로 작성)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // [STEP 3] 인스턴스 생성 및 리팩터링
    // ---------------------------------------------------------
    
    // <Version 1: 개별 변수>
    // let width1 = 30;
    // let height1 = 50;
    // -> 문제점: width와 height가 서로 연관된 데이터라는 것을 알기 힘듦.

    // <Version 2: 튜플>
    // let rect1 = (30, 50);
    // -> 문제점: rect1.0, rect1.1 처럼 인덱스로 접근해야 해서 의미가 불분명함.

    // <Version 3: 구조체>
    // -> 해결: 필드에 이름(width, height)이 생겨 의미가 명확해짐.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 연관 함수(생성자)를 사용하여 정사각형 생성
    let rect2 = Rectangle::square(20); 
    let rect3 = Rectangle::square(60);

    // [STEP 4] 기능 사용
    // ---------------------------------------------------------
    
    // 4-1. Debug 출력
    // 구조체는 {} 대신 {:?} 또는 {:#?}를 사용해야 합니다.
    println!("rect1 정보: {:#?}", rect1); 

    // 4-2. 메서드 호출 (Method Syntax)
    // rect1.area() 호출 시 Rust가 자동으로 참조(&)를 생성하여 전달합니다.
    println!(
        "rect1의 면적: {}",
        rect1.area() // == Rectangle::area(&rect1)
    );

    // 4-3. 메서드에 다른 인스턴스 전달
    println!("rect1은 rect2를 포함할 수 있나요? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3를 포함할 수 있나요? {}", rect1.can_hold(&rect3));
}