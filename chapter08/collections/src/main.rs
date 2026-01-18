use std::collections::HashMap;

fn main() {
    // [ Vector ]

    // 비어 있는 새 벡터 생성 및 타입 명시
    let mut v: Vec<i32> = Vec::new();

    // push()를 이용한 요소 추가 (mut일 때만 가능)
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // vec! 매크로를 이용한 초기값 포함 벡터 생성 및 타입 추론
    let v2 = vec![1, 2, 3, 4, 5];

    // 인덱싱(&[])을 이용한 요소 접근 (범위 초과 시 패닉 발생)
    let third: &i32 = &v2[2];
    println!("인덱싱으로 접근한 세 번째 요소: {third}");

    // get 메서드를 이용한 안전한 접근 (Option<&T> 반환)
    match v2.get(2) {
        Some(value) => println!("get으로 접근한 세 번째 요소: {value}"),
        None => println!("해당 요소가 존재하지 않음"),
    }

    // for 루프를 이용한 불변 참조자 순회
    println!("벡터 요소 순회 출력:");
    for i in &v {
        println!("{i}");
    }

    // for 루프와 역참조 연산자(*)를 이용한 가변 참조자 순회 및 값 수정
    for i in &mut v {
        *i += 50;
    }

    // Enum을 활용하여 벡터에 여러 타입 저장 명시
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    // [ String ]

    // 빈 String 생성(String::from) 및 리터럴로부터 String 생성(to_string)
    let mut s1 = String::new();

    let data = "initial contents";
    let s2 = data.to_string();

    let mut s3 = String::from("foo");

    // push_str(str 추가) 및 push(char 추가)를 이용한 수정
    s3.push_str("bar");
    s3.push('!'); 

    // + 연산자를 이용한 결합 (s2는 소유권 이동, s3는 참조자 사용)
    let s4 = s2 + &s3; 
    // println!("{s2}"); // s2는 소유권이 이동되어 사용 불가

    // format! 매크로를 이용한 복잡한 문자열 결합 (소유권 유지)
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let game = format!("{tic}-{tac}-{toe}");
    println!("format! 결과: {game}");

    // 문자열 슬라이싱 (바이트 범위 지정, 문자 경계 주의 필요)
    let hello = "안녕하세요";
    let hello_slice = &hello[0..3];
    println!("문자열 슬라이스: {hello_slice}");

    // chars() 메서드를 이용한 개별 유니코드 스칼라 값 순회
    print!("chars() 순회: ");
    for c in "नमस्ते".chars() {
        print!("{c} ");
    }
    println!();


    // [ Hash Map ]

    // 새 해시 맵 생성 및 데이터 삽입(insert)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get, copied, unwrap_or를 이용한 안전한 값 추출
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue 팀의 점수: {score}");

    // 해시 맵 내의 소유권 이동 명시 (String 타입 등)
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{field_name}"); // 소유권이 map으로 이동되어 사용 불가

    // entry API를 이용한 값 존재 여부 확인 및 없을 때만 삽입 (entry + or_insert)
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(25);

    // 기존 값을 참조하여 값 업데이트 (단어 개수 세기 예제)
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1; // 가변 참조자를 역참조하여 값 수정
    }
    println!("단어 개수 결과: {:?}", word_count);

} 