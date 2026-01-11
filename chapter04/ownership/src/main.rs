fn main() {
    // 1. 스코프(Scope) 자동 해제
    {
        let s = String::from("hello"); 
        println!("Scope internal: {s}");
    } // s drop (메모리 해제)

    // 2. 이동(Move) vs 복사(Copy)
    // [Heap 데이터] Move: 소유권이 완전히 넘어감 
    let s1 = String::from("hello");
    let s2 = s1; 
    // println!("{s1}"); // 사용 불가 (Move되어 s1 무효화)
    println!("s2 owns the heap data: {s2}");

    // [Stack 데이터] Copy: 값이 복제되어 둘 다 살아남음
    let x = 5;
    let y = x; 
    println!("x and y are separate copies: {x}, {y}");

    // 3. 함수를 통한 소유권 전달
    let s3 = String::from("rust");
    takes_ownership(s3); // s3 소유권이 함수로 'Move'
    // println!("{s3}"); // 사용 불가

    let z = 10;
    makes_copy(z); // 정수 z는 'Copy'되어 전달됨
    println!("Main still has z: {z}");

    // 4. 반환(Return)을 통한 소유권 회수
    let s4 = gives_ownership(); 
    println!("Received ownership: {s4}");

    let s5 = String::from("hello");
    // 소유권을 주고(s5) -> 다시 돌려받는(s6) 번거로운 과정
    let (s6, len) = calculate_length(s5); 
    println!("The length of '{s6}' is {len}.");

    let s7 = String::from("hello");
    takes_ownership(s7.clone()); // clone을 사용하여 원본 보존 가능
    println!("Main still has s7 after clone-passing: {s7}");
}

fn takes_ownership(some_string: String) {
    println!("Function took ownership: {some_string}");
} // some_string 메모리 해제

fn makes_copy(some_integer: i32) {
    println!("Function got a copy: {some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 소유권을 호출자에게 반환
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}