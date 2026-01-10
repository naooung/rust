fn main() {
    /* 함수 기본 호출
    */
    println!("--- Function Basics & Parameters ---");
    another_function(5);
    print_labeled_measurement(10, 'm');

    /* 구문과 수식
       구문: 값을 반환 X, 세미콜론(;) 필요
       수식: 값을 계산, 세미콜론(;) X
    */
    println!("\n--- Statements vs Expressions ---");
    let y = {
        let x = 3;
        x + 1  // 세미콜론이 없음 -> 수식에 해당
    };
    println!("The value of y is: {y}");

    /* 반환값이 있는 함수
       수식의 뒤에 세미콜론은 붙이면 빈 값인 유닛 타입()을 반환하여 타입 불일치 오류 발생
    */
    println!("\n--- Return Values ---");
    let x = five();
    let z = plus_one(x);
    println!("Five function result: {x}");
    println!("Plus one function result: {z}");
}

// 매개변수가 하나인 함수
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// 매개변수가 여러 개인 함수
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 반환값만 있는 함수
fn five() -> i32 {
    5 // 세미콜론 X (있으면 구문이 되므로 에러 발생)
}

// 매개변수와 반환값이 모두 있는 함수
fn plus_one(x: i32) -> i32 {
    x + 1
}