fn main() {
    // 1. 불변 참조 (&)
    // 소유권을 넘기지 않고 '빌려주기'만 함 (읽기 전용)
    let s1 = String::from("hello");
    let len = calculate_len_ref(&s1);
    println!("The length of '{s1}' is {len}."); // s1은 소유권을 잃지 않아 계속 사용 가능

    // 2. 가변 참조 (&mut) 
    // 빌려온 값을 직접 수정할 수 있게 함
    let mut s2 = String::from("hello");
    change_ref(&mut s2); // 수정 권한까지 빌려줌
    println!("Changed string: {s2}");

    // 3. 가변 참조의 제약: 한 번에 하나만 가능 (데이터 경합 방지) 
    let mut s3 = String::from("data");
    let r1 = &mut s3;
    // let r2 = &mut s3; // 가변 참조가 살아있는 동안(마지막으로 사용되기 전) 또 다른 가변 참조 불가
    println!("r1: {r1}"); 

    // 4. 불변과 가변의 혼용 금지
    let mut s4 = String::from("mixed");
    let r3 = &s4;
    let r4 = &s4;
    // let r5 = &mut s4; // 읽기 도중 변경 불가로 오류 발생
    println!("r3: {r3}, r4: {r4}");
    
    // r3, r4의 마지막 사용이 끝났으므로, 이제는 가변 참조 생성 가능 (NLL 규칙)
    let r6 = &mut s4; 
    println!("r6: {r6}");
}

fn calculate_len_ref(s: &String) -> usize {
    s.len()
} 

fn change_ref(some_string: &mut String) {
    some_string.push_str(", world");
}