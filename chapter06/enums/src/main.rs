// [STEP 1] 열거형 정의 (Enums Definition)
// 예제 1: 각 변형(Variant)이 다른 타입과 양의 데이터를 가질 수 있음
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // 4개의 u8 튜플
    V6(String),         // 1개의 String 값을 가짐
}

// 예제 2: 다양한 형태의 데이터를 가진 열거형 (구조체, 튜플 등 혼합)
enum Message {
    Quit,                    
    Move { x: i32, y: i32 },    // 이름 있는 필드 (구조체)
    Write(String),              // 단일 데이터 (튜플)
    ChangeColor(i32, i32, i32), // 여러 데이터 (튜플)
}

// 예제 3: 패턴 매칭 '값 바인딩'
#[derive(Debug)] // {:?} 출력을 위해
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter는 UsState를 가짐
}

fn main() {
    // [STEP 2] 열거형 인스턴스 생성
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // [STEP 3] Option<T> 열거형 (Null 대체 방법)
    // Some은 값 O, None은 값 X
    let five = Some(5);
    let absent_number: Option<i32> = None; // None일 땐 타입 명시 필요

    // [STEP 4] 함수 호출 및 match 실행
    // 4-1. Coin 예제 (값을 꺼내서 바인딩)
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin); 
    println!("동전의 가치: {}센트", value);

    // 4-2. Option 예제 (매칭 연산)
    let six = plus_one(five);
    let none = plus_one(absent_number);
    println!("5 + 1 = {:?}", six);   // Some(6)
    println!("None + 1 = {:?}", none); // None

    // [STEP 5] 포괄적 패턴 (Catch-all)과 _
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // 값을 사용할 때 (other 변수에 바인딩)
        // _ => reroll(), // 값을 무시할 때 (와일드카드)
    }

    // [STEP 6] if let
    let config_max = Some(3u8);
    
    // match로 썼을 때 (보일러플레이트 발생)
    /*
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), // 매번 똑같이 사용하는 보일러플레이트 발생 }
    */

    // if let으로 썼을 때
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // [STEP 7] if ... else
    let coin_for_state = Coin::Quarter(UsState::Alabama);
    describe_state_quarter(coin_for_state);
}

// [Match 예제] 동전 가치 계산 (Coin::Quarter의 state 바인딩)
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// [Option 예제] 값이 있으면 +1, 없으면 None (포괄성 검사)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None 처리 안 하면 컴파일 에러 (포괄성 위반)
        None => None,
        // i는 그냥 i32 정수에 해당하므로 연산 후 다시 Some으로 감싸야 함
        Some(i) => Some(i + 1),
    }
}

// [let...else 예제] if let과 비교하여 들여쓰기를 줄이는 방법
fn describe_state_quarter(coin: Coin) -> Option<String> {
    // 1. 패턴 매칭을 시도
    // 2. 성공하면 state 변수가 생성되어 아래 코드에서 사용
    // 3. 실패하면 else 블록이 실행되고 함수가 즉시 종료(return)
    let Coin::Quarter(state) = coin else {
        println!("Quarter가 아닙니다.");
        return None;
    };

    // 여기부터는 coin이 무조건 Quarter임이 보장됨 (Happy Path)
    if state.existed_in(1900) {
        println!("{:?}는 꽤 오래된 주입니다!", state);
        Some(String::from("Old"))
    } else {
        println!("{:?}는 비교적 새로운 주입니다.", state);
        Some(String::from("New"))
    }
}

// 주사위 예제를 위한 더미 함수
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}