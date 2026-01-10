fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// Shadowing 장점
// + 기존 메모리 위치의 값을 수정하는 것이 아닌 스탭 메모리의 새로운 위치에 값 할당
// 1. 값 변환 + 최종값 불변성 유지
// 2. 타입 변경 가능 (mut은 타입 변경 불가)