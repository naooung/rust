fn main() {
    // 1. 문자열 슬라이스 (&str)
    let s = String::from("hello world");

    let hello = &s[0..5];  // 0~4 인덱스 부분 참조
    let world = &s[6..11]; // 6~10 인덱스 부분 참조
    println!("Slices: {hello}, {world}");

    // 2. 슬라이스를 이용한 안전한 단어 찾기
    let s_safe = String::from("hello rust");
    
    // &s_safe는 String 전체의 슬라이스(&str)로 자동 변환되어 전달됨
    let word = first_word(&s_safe); 

    // s_safe.clear(); // word가 s_safe의 일부분을 가르키기 때문(불변 참조)에 clear(가변 참조) 요청 불가
    
    println!("The first word is: {word}"); 

    // 3. 문자열 리터럴과 범용성
    let literal = "hello world";
    let word2 = first_word(literal); 
    println!("From literal: {word2}");

    // 4. 배열 슬라이스
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3]; // &[i32] 타입
    println!("Array slice: {:?}", a_slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b는 공백 문자의 바이트값(32u8)을 의미 
            return &s[0..i];
        }
    }
    &s[..] // 공백이 없으면 전체 반환
}